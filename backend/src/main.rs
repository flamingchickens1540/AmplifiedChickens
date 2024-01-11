use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    routing::{get, post},
    Extension, Router,
};
use cookie::Key;
use dotenv::dotenv;
use oauth2::basic::BasicClient;
use reqwest::Client as ReqwestClient;
use socketioxide::layer::SocketIoLayer;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;
// A scrapped queuing rewrite using an api
mod auth;
mod model;
mod ws;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_uri = std::env::var("DATABASE_URI").expect("DATABASE_URI is not set");
    let db = sqlx::PgPool::connect_lazy(db_uri.as_str()).unwrap();
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed migrations :(");

    dotenv().ok();
    let server_host = std::env::var("SERVER_HOST").expect("SERVER_HOST is not set");
    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT is not set");

    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let (ws_layer, io) = ws::create_layer();

    io.ns("/", ws::on_connect);

    let ctx = ReqwestClient::new();

    let state = model::AppState {
        db, // Database
        ctx,
        key: Key::generate(), // Cookie key
    };
    let oauth_client = auth::build_oauth_client();
    let router = init_router(ws_layer, oauth_client);
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", server_host, server_port).as_str())
            .await
            .unwrap();

    info!("Starting Server");
    info!("Listening on port {}", server_port);

    axum::serve(listener, router).await.unwrap();
    Ok(())
}

fn init_router(ws: SocketIoLayer, oauth_client: BasicClient) -> Router {
    let auth = Router::new().route("/auth/scout", get(auth::google_callback));
    //let protected_router = Router::new().route("/", get(oauth::protected)).route_layer(
    //    middleware::from_fn_with_state(state.clone(), oauth::check_authenticated),
    //);

    let frontend = front_public_route().layer(Extension(oauth_client));
    Router::new()
        //.nest("/auth", auth::create_api_router())
        .merge(frontend)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive()) // Enable CORS policy
                .layer(ws),
        )
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

#[allow(clippy::unused_async)]
async fn handle_error() -> (StatusCode, &'static str) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong accessing static files...",
    )
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
