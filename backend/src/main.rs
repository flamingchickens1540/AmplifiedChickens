use axum::{
    handler::HandlerWithoutStateExt, http::StatusCode, middleware, response::Html, routing::get,
    Extension, Router,
};
use cookie::Key;
use dotenv::dotenv;
use oauth2::basic::BasicClient;
use reqwest::Client as ReqwestClient;
use socketioxide::layer::SocketIoLayer;

use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

mod auth;
mod error;
mod model;
mod ws;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let server_host = std::env::var("SERVER_HOST").expect("SERVER_HOST is not set");
    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT is not set");

    let client_id = std::env::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID from .env");
    let client_secret =
        std::env::var("GOOGLE_CLIENT_SECRET").expect("Missing GOOGLE_CLIENT_SECRET from .env");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db: model::Db = model::Db::new(db_url).await.unwrap();

    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let (ws_layer, io) = ws::create_layer();

    io.ns("/", ws::on_connect);

    let ctx = ReqwestClient::new();

    let state = model::AppState {
        db, // Database
        ctx,
        key: Key::generate(), // Cookie key
    };
    let oauth_client = auth::build_oauth_client(client_id.clone(), client_secret);
    let router = init_router(state, ws_layer, oauth_client, client_id);
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", server_host, server_port).as_str())
            .await
            .unwrap();

    info!("Starting Server");
    info!("Listening on port {}", server_port);

    axum::serve(listener, router).await.unwrap();
    Ok(())
}

fn init_router(
    state: model::AppState,
    _ws: SocketIoLayer,
    oauth_client: BasicClient,
    oauth_id: String,
) -> Router {
    // this router has state
    let auth = Router::new().route("/", get(auth::google_callback));

    let unprotected: Router<model::AppState> = Router::new()
        .route("/", get(homepage))
        .layer(Extension(oauth_id));

    let protected =
        Router::new()
            .route("/", get(protected))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth::user_auth,
            ));

    let admin = Router::new()
        .route("/", get(admin))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::admin_auth,
        ));

    // this router doesn't
    Router::new()
        .nest("/auth/", auth)
        .nest("/protected", protected)
        .nest("/admin", admin)
        .nest("/", unprotected)
        .layer(Extension(oauth_client))
        .with_state(state)
    //.layer(
    //    ServiceBuilder::new()
    //        .layer(CorsLayer::permissive()) // Enable CORS policy
    //        .layer(ws),
    //)
}

// FrontEnd Routing
// FrontEnd to server svelte build bundle, css and index.html from public folder
pub fn front_public_route() -> Router {
    let front_public = "./frontend/dist"; //std::env::var("FRONT_PUBLIC").expect("FRONT_PUBLIC is not set");
    Router::new()
        .fallback_service(
            ServeDir::new(front_public).not_found_service(handle_error.into_service()),
        )
        .layer(TraceLayer::new_for_http())
}

async fn handle_error() -> (StatusCode, &'static str) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong accessing static files...",
    )
}

#[axum::debug_handler]
async fn homepage(Extension(oauth_id): Extension<String>) -> Html<String> {
    Html(format!("<p>Welcome!</p>
    
    <a href=\"https://accounts.google.com/o/oauth2/v2/auth?scope=openid%20profile%20email&client_id={oauth_id}&response_type=code&redirect_uri=http://localhost:3007/auth/\">
    Click here to sign into Google!
     </a>"))
}

#[axum::debug_handler]
async fn protected(Extension(user): Extension<model::User>) -> Html<String> {
    Html(format!("<p>Welcome {}<p>", user.name))
}

#[axum::debug_handler]
async fn admin(Extension(user): Extension<model::User>) -> Html<String> {
    Html(format!("<p>Welcome Admin {}<p>", user.name))
}

// With a form of auth
// // ********
// // BACK END
// // ********
// // Back end server built form various routes that are either public, require auth, or secure login
// pub fn backend<Store: SessionStore>(
//     session_layer: SessionManagerLayer<Store>,
//     shared_state: Arc<store::Store>,
// ) -> Router {
//     let session_service = ServiceBuilder::new()
//         .layer(HandleErrorLayer::new(|_: BoxError| async {
//             StatusCode::BAD_REQUEST
//         }))
//         .layer(session_layer);

//     // could add tower::ServiceBuilder here to group layers, especially if you add more layers.
//     // see https://docs.rs/axum/latest/axum/middleware/index.html#ordering
//     Router::new()
//         .merge(back_public_route())
//         .merge(back_auth_route())
//         .merge(back_token_route(shared_state))
//         .layer(session_service)
// }
