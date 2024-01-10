use axum::{
    Router, http::StatusCode, handler::HandlerWithoutStateExt
};
use tower_http::{
    services::ServeDir, 
    trace::TraceLayer 
};


const FRONT_PUBLIC: &str = "./frontend/dist";
const SERVER_PORT: &str = "8080";
const SERVER_HOST: &str = "0.0.0.0";

// #[tokio::main]
// async fn main() {
//     let app = init_router();
//         // .route("/", get(hello_world()));
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(); // 0.0.0.0 makes it compatable with docker images
//     axum::serve(listener, app).await.unwrap();
// }

// fn hello_world() -> &'static str {
//     "Hello World!"
// }

#[tokio::main]
async fn main() {
    let router = init_router();
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", SERVER_HOST, SERVER_PORT).as_str()).await.unwrap();
    axum::serve(listener, router).await.unwrap()
}

fn init_router() -> Router {
    Router::new()
        .merge(front_public_route())
        // .merge(backend(session_layer, shared_state))
    // Router::new().nest_service("/", 
    //     ServeDir::new("../../frontend/dist")
    //     .not_found_service(ServeFile::new("../../frontend/dist/index.html")) // fallback
    // )
}

// *********
// FRONT END
// *********
// Front end to server svelte build bundle, css and index.html from public folder
pub fn front_public_route() -> Router {
    Router::new()
        .fallback_service(
            ServeDir::new(FRONT_PUBLIC)
            .not_found_service(handle_error.into_service()),
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