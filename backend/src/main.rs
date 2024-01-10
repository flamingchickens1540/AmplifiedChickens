use axum::{handler::HandlerWithoutStateExt, http::StatusCode, Router};
use dotenv::dotenv;
use reqwest::Client as ReqwestClient;
use socketioxide::layer::SocketIoLayer;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;
// A scrapped queuing rewrite using an api
//mod auth;
mod model;
mod ws;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //dotenv().ok();
    let server_host = "0.0.0.0"; //std::env::var("SERVER_HOST").expect("SERVER_HOST is not set");
    let server_port = "3000"; //std::env::var("SERVER_PORT").expect("SERVER_PORT is not set");
    info!("{}", server_host);
    info!("{}", server_port);
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let (ws_layer, io) = ws::create_layer();

    io.ns("/", ws::on_connect);

    //let _oauth_id = dotenv::var("GOOGLE_OAUTH_CLIENT_ID").unwrap();
    //let _oauth_secret = dotenv::var("GOOGLE_OAUTH_CLIENT_SECRET").unwrap();

    let _ctx = ReqwestClient::new();

    //    let state = model::AppState {
    //      db, // Database
    //    ctx,
    //  key: Key::generate(), // Cookie key
    //};

    let router = init_router(ws_layer);

    info!("Starting Server");

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", server_host, server_port).as_str())
            .await
            .unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}

fn init_router(layer: SocketIoLayer) -> Router {
    Router::new()
        //.nest("/auth", auth::create_api_router())
        .merge(front_public_route())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive()) // Enable CORS policy
                .layer(layer),
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
