use axum::{
    extract::{DefaultBodyLimit, Host},
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    middleware,
    response::Html,
    response::Redirect,
    routing::{get, post},
    BoxError, Extension, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use cookie::Key;
use dotenv::dotenv;
use oauth2::basic::BasicClient;
use reqwest::Client as ReqwestClient;
use socketioxide::layer::SocketIoLayer;
use std::{net::SocketAddr, path::PathBuf};

use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

mod auth;
mod error;
mod model;
mod upload;
mod ws;

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

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

    let ctx = ReqwestClient::new();

    let state = model::AppState {
        db, // Database
        ctx,
        key: Key::generate(), // Cookie key
    };
    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file("cert.pem", "key.pem")
        .await
        .unwrap();
    let ports = Ports {
        http: 7878,
        https: 3007,
    };
    let addr = SocketAddr::from(([0, 0, 0, 0], 3007));

    tokio::spawn(redirect_http_to_https(ports));

    let router = init_router(state);

    info!("Starting Server");
    info!("Listening on {}", addr);

    axum_server::bind_rustls(addr, config)
        .serve(router.into_make_service())
        .await
        .unwrap();

    info!("Starting Server");
    info!("Listening on port {}", server_port);

    Ok(())
}

fn init_router(state: model::AppState) -> Router {
    let max_image_size: usize = std::env::var("MAX_IMAGE_SIZE")
        .expect("MAX_IMAGE_SIZE not set")
        .parse()
        .unwrap_or(50)
        * 1024
        * 1024;
    let unprotected: Router<model::AppState> = Router::new().route("/", get(homepage));

    let protected =
        Router::new()
            .route("/", get(protected))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth::user_auth,
            ));

    Router::new()
        .nest("/", unprotected)
        .nest("/protected", protected)
        .route("/image/:image", get(upload::image))
        .route("/upload", post(upload::upload))
        .layer(DefaultBodyLimit::max(max_image_size))
        .route("/auth/slack", get(auth::fake_callback))
        .with_state(state)
        .layer(
            tower::ServiceBuilder::new().layer(CorsLayer::permissive()), // Enable CORS policy
        )
}

#[axum::debug_handler]
async fn homepage() -> Html<String> {
    let slack_redirect_url = dotenv::var("SLACK_REDIRECT_URL").expect("REDIRECT_URL not set");
    let google_redirect_url =
        dotenv::var("GOOGLE_REDIRECT_URL").expect("GOOGLE REDIRECT_URL not set");
    let scopes = "identity.basic,identity.email,email,profile";
    let oauth_id = dotenv::var("SLACK_CLIENT_ID").expect("SLACK_CLIENT_ID no set");

    Html(format!("<p>Welcome!</p>

    <a href=\"https://slack.com/openid/connect/authorize?scope=openid%20profile%20email&amp;response_type=code&amp;redirect_uri=https%3A%2F%2Flocalhost%3A3007%2Fauth%2Fslack&amp;client_id=10700242916.6475903895858\" style=\"align-items:center;color:#000;background-color:#fff;border:1px solid #ddd;border-radius:4px;display:inline-flex;font-family:Lato, sans-serif;font-size:16px;font-weight:600;height:48px;justify-content:center;text-decoration:none;width:256px\"><svg xmlns=\"http://www.w3.org/2000/svg\" style=\"height:20px;width:20px;margin-right:12px\" viewBox=\"0 0 122.8 122.8\"><path d=\"M25.8 77.6c0 7.1-5.8 12.9-12.9 12.9S0 84.7 0 77.6s5.8-12.9 12.9-12.9h12.9v12.9zm6.5 0c0-7.1 5.8-12.9 12.9-12.9s12.9 5.8 12.9 12.9v32.3c0 7.1-5.8 12.9-12.9 12.9s-12.9-5.8-12.9-12.9V77.6z\" fill=\"#e01e5a\"></path><path d=\"M45.2 25.8c-7.1 0-12.9-5.8-12.9-12.9S38.1 0 45.2 0s12.9 5.8 12.9 12.9v12.9H45.2zm0 6.5c7.1 0 12.9 5.8 12.9 12.9s-5.8 12.9-12.9 12.9H12.9C5.8 58.1 0 52.3 0 45.2s5.8-12.9 12.9-12.9h32.3z\" fill=\"#36c5f0\"></path><path d=\"M97 45.2c0-7.1 5.8-12.9 12.9-12.9s12.9 5.8 12.9 12.9-5.8 12.9-12.9 12.9H97V45.2zm-6.5 0c0 7.1-5.8 12.9-12.9 12.9s-12.9-5.8-12.9-12.9V12.9C64.7 5.8 70.5 0 77.6 0s12.9 5.8 12.9 12.9v32.3z\" fill=\"#2eb67d\"></path><path d=\"M77.6 97c7.1 0 12.9 5.8 12.9 12.9s-5.8 12.9-12.9 12.9-12.9-5.8-12.9-12.9V97h12.9zm0-6.5c-7.1 0-12.9-5.8-12.9-12.9s5.8-12.9 12.9-12.9h32.3c7.1 0 12.9 5.8 12.9 12.9s-5.8 12.9-12.9 12.9H77.6z\" fill=\"#ecb22e\"></path></svg>Sign in with Slack</a>
    <a href=\"https://accounts.google.com/o/oauth2/v2/auth?scope=openid%20profile%20email&client_id={oauth_id}&response_type=code&redirect_uri={google_redirect_url}\">
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

#[allow(dead_code)]
async fn redirect_http_to_https(ports: Ports) {
    fn make_https(host: String, uri: Uri, ports: Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], ports.http));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
}
