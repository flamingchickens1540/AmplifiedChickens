use crate::admin::get_user_helper;
use crate::{
    error,
    model::{AppState, Db},
};
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use base64ct::{Base64UrlUnpadded, Encoding};
use http::{HeaderMap, Uri};
use hyper::StatusCode;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use web_push_native::{
    jwt_simple::algorithms::ES256KeyPair, p256::elliptic_curve::PublicKey, Auth, WebPushBuilder,
};

pub async fn send_web_push(db: &Db, token: String) -> Result<(), (StatusCode, String)> {
    // THis is not being called
    info!("Send webpush called");
    let user = get_user_helper(db, token).await?;

    let VAPID_PRIVATE = std::env::var("VAPID_PRIVATE").expect("VAPID_PRIVATE not set");
    let ENDPOINT = user.endpoint.clone().unwrap();
    let P256DH = user.p256dh.clone().unwrap();
    let AUTH = user.auth.clone().unwrap();

    let vapid = Base64UrlUnpadded::decode_vec(&VAPID_PRIVATE).unwrap();
    let endpoint: Uri = ENDPOINT.parse().unwrap();
    let P256DH = Base64UrlUnpadded::decode_vec(&P256DH).unwrap();
    let AUTH = Base64UrlUnpadded::decode_vec(&AUTH).unwrap();

    let key_pair = ES256KeyPair::from_bytes(&vapid).unwrap();
    let ua_public = PublicKey::from_sec1_bytes(&P256DH).unwrap();
    let ua_auth = Auth::clone_from_slice(&AUTH);

    let message = json!({
      "title": "Scouting Notification",
      "body": "Time to scout!"
    });

    let builder = WebPushBuilder::new(endpoint, ua_public, ua_auth)
        .with_vapid(&key_pair, "colburna@team1540.catlin.edu")
        .build(message.to_string())
        .unwrap()
        .map(axum::body::Body::from);

    info!("builder: {:?}", builder);

    let https = hyper_tls::HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build(https);
    info!("Client sent push notification: {:?}", client);
    let test = client.request(builder).await.unwrap();
    info!("test: {:?}", test);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VapidKey {
    pub public_key: String,
}

#[axum::debug_handler]
pub async fn vapid() -> axum::Json<VapidKey> {
    info!("Getting VAPID key");
    let public_key: String = std::env::var("VAPID_PUBLIC").expect("VAPID key not set");

    let key = VapidKey {
        public_key: public_key,
    };

    Json(key)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebPush {
    pub endpoint: String,
    pub keys: Keys,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct Keys {
    auth: String,
    p256dh: String,
}

pub async fn register(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(webpush): Json<WebPush>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let access_token = match headers.get("x-access-token") {
        Some(token) => token
            .to_str()
            .expect("access_token was an invalid string")
            .to_string(),
        None => {
            error!("User attempted to register without access_token");
            return Err((
                StatusCode::UNAUTHORIZED,
                "Unauthorzed, no access_token provided".to_string(),
            ));
        }
    };

    let user = get_user_helper(&state.db, access_token.clone()).await?;
    if let Err(e) = sqlx::query!(
        "UPDATE \"Users\" SET endpoint = $2, p256dh = $3, auth = $4 WHERE id = $1",
        user.id,
        webpush.endpoint,
        webpush.keys.p256dh,
        webpush.keys.auth
    )
    .execute(&state.db.pool)
    .await
    {
        error!("Error trying to update WebPush details: {e}");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error trying to update WebPush details: {e}"),
        ));
    };
    Ok(())
}
