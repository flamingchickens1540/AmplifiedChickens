use axum::{
    extract::{DefaultBodyLimit, Host},
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    BoxError, Json, Router,
};

use axum_server::tls_rustls::RustlsConfig;
use dotenv::dotenv;

use reqwest::Client as ReqwestClient;
use serde_json::json;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

use self::model::TeamMatch;

mod auth;
mod error;
mod model;
mod queue;
mod submit;
mod upload;

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db: model::Db = model::Db::new(db_url).await.unwrap();

    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let ctx = ReqwestClient::new();

    let queue = Arc::new(Mutex::new(model::RoboQueue {
        match_keys: vec![],
        robots: vec![],
        scouts: vec![],
    }));

    let state = model::AppState {
        db, // Database
        ctx,
        queue,
        team_match_upstreams: Arc::new(Mutex::new(vec![])),
    };
    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file("cert.pem", "key.pem")
        .await
        .expect("Missing cert.pem and key.pem files");
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

    Ok(())
}

fn init_router(state: model::AppState) -> Router {
    let max_image_size: usize = std::env::var("MAX_IMAGE_SIZE")
        .expect("MAX_IMAGE_SIZE not set")
        .parse()
        .unwrap_or(50)
        * 1024
        * 1024;

    Router::new()
        .route("/health", get(health))
        .route("/dummyData", get(dummy_data))
        .route("/auth/check", get(auth::check_auth))
        .route("/submit/image/:image", get(upload::image))
        .route("/submit/upload", post(upload::upload))
        .layer(DefaultBodyLimit::max(max_image_size))
        .route("/auth/slack", get(auth::slack_callback))
        .route("/submit/pit", post(submit::submit_pit_data))
        .route("/submit/match", post(submit::submit_team_match))
        //.route("/admin/getUser/single", get(queue::get_user))
        .route("/admin/newMatch/manual", post(queue::new_match_manual))
        .route("/admin/newMatch/auto", post(queue::new_match_auto))
        .route("/admin/newEvent", post(queue::new_event))
        .route("/admin/getUser/all", get(queue::get_scouts_and_scouted))
        .route("/admin/getUser/queued", get(queue::get_queued_scouts))
        .route("/scout/inQueue", get(queue::in_queue))
        .route("/scout/queueUser", post(queue::queue_user))
        .with_state(state)
        .layer(
            tower::ServiceBuilder::new().layer(CorsLayer::permissive()), // Enable CORS policy
        )
}

async fn health() -> Result<impl IntoResponse, Infallible> {
    Ok(())
}

async fn dummy_data() -> Result<Json<TeamMatch>, Infallible> {
    let team_match = TeamMatch::default();

    Ok(Json(team_match))
}

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
