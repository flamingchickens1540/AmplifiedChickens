use axum::{
    handler::HandlerWithoutStateExt, http::StatusCode, middleware, response::Html, routing::get,
    Extension, Router,
};
use cookie::Key;
use dotenv::dotenv;
use oauth2::basic::BasicClient;
use reqwest::Client as ReqwestClient;
use socketioxide::layer::SocketIoLayer;

use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

mod auth;
mod error;
mod model;
mod ws;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let server_host = dotenv::var("SERVER_HOST").expect("SERVER_HOST is not set");
    let server_port = dotenv::var("SERVER_PORT").expect("SERVER_PORT is not set");

    let client_id = dotenv::var("SLACK_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID from .env");
    let client_secret =
        dotenv::var("SLACK_CLIENT_SECRET").expect("Missing GOOGLE_CLIENT_SECRET from .env");

    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not set");
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
    let oauth_client = auth::build_slack_oauth_client(client_id.clone(), client_secret);
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
    let auth = Router::new().route("/slack", get(auth::slack_callback));

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

    //let frontend = front_public_route().layer(Extension(oauth_client));

    // this router doesn't
    Router::new()
        .nest("/auth", auth)
        .nest("/protected", protected)
        .nest("/admin", admin)
        .nest("/", unprotected)
        .layer(Extension(oauth_client))
        .with_state(state)
        .layer(
            tower::ServiceBuilder::new().layer(CorsLayer::permissive()), // Enable CORS policy
        )
}

// FrontEnd Routing
// FrontEnd to server svelte build bundle, css and index.html from public folder
pub fn front_public_route() -> Router {
    let front_public = "./frontend/dist"; //dotenv::var("FRONT_PUBLIC").expect("FRONT_PUBLIC is not set");
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
    let redirect_url = dotenv::var("SLACK_REDIRECT_URL").expect("REDIRECT_URL not set");
    let scopes = "identity.basic,identity.email";

    let url = format!(
        "https://slack.com/oauth/v2/authorize?client_id={}&user_scope={}&redirect_uri={}",
        oauth_id, scopes, redirect_url
    );
    Html(format!("<p>Welcome!</p>
    <a href={url}><img alt=\"Login with Slack\" height=\"40\" width=\"139\" src=\"https://platform.slack-edge.com/img/add_to_slack.png\" srcSet=\"https://platform.slack-edge.com/img/add_to_slack.png 1x, https://platform.slack-edge.com/img/add_to_slack@2x.png 2x\"/></a>"))
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
