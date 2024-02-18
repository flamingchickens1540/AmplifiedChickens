use std::convert::Infallible;

use crate::{
    error,
    model::{self, AppState, Db, User},
};
use axum::{
    extract::{Json, State},
    response::{IntoResponse, Response},
};
use axum_server::accept;
use futures::TryFutureExt;
use http::{HeaderMap, Uri};
use hyper::{header, StatusCode};
// use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use web_push::{ContentEncoding, IsahcWebPushClient, SubscriptionInfo, VapidSignatureBuilder, WebPushMessageBuilder};
use web_push_native::{
    jwt_simple::algorithms::{ECDSAP256KeyPairLike, ES256KeyPair},
    p256::{
        elliptic_curve::{consts::P256, generic_array::GenericArray, PublicKey}, pkcs8::der::pem::decode, NistP256
    },
    Auth, WebPushBuilder,
};
use base64ct::{Base64UrlUnpadded, Encoding};


use crate::queue::get_user_helper;

pub async fn send(State(state): State<AppState>) -> axum::Json<VapidKey> {
    info!("Send");
    let user = send_web_push(
        &state.db,
        "xoxp-10700242916-1354597325236-6564689263399-dd590fd79ad145f288f43c8d6b379d31".to_string(),
    );

    user.await.unwrap();

    let key = VapidKey {
        public_key: "test".to_string(),
    };

    Json(key)
}

pub async fn send_web_push(db: &Db, token: String) -> Result<(), (StatusCode, String)> {
    info!("Send web push");



    // let user = get_user_helper(&db, token).await?;
    
    // let endpoint = user.endpoint;
    // let p256dh = user.p256dh;
    // let auth = user.auth;

    // //You would likely get this by deserializing a browser `pushSubscription` object via serde.  
    // let subscription_info = SubscriptionInfo::new(
    //     endpoint,
    //     p256dh,
    //     auth
    // );

    // //Read signing material for payload.
    // let file = File::open("private.pem").unwrap();
    // let mut sig_builder = VapidSignatureBuilder::from_pem(file, &subscription_info)?.build()?;

    // //Now add payload and encrypt.
    // let mut builder = WebPushMessageBuilder::new(&subscription_info);
    // let content = "Encrypted payload to be sent in the notification".as_bytes();
    // builder.set_payload(ContentEncoding::Aes128Gcm, content);
    // builder.set_vapid_signature(sig_builder);

    // let client = IsahcWebPushClient::new()?;

    // //Finally, send the notification!
    // client.send(builder.build()?).await?;
    // Ok(())
 
        
    
    // let builder: WebPushBuilder = match sqlx::query_as(
    //     "SELECT endpoint, p256dh, auth FROM \"Users\" WHERE access_token = $1",
    // )
    // .bind(token)
    // .fetch_one(&(db.pool))
    // .await
    // {
    //     Ok(res) => res,
    //     Err(e) => {
    //         error!("{}", e);
    //         return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get user".to_string()));
    //     }
    // };

    // info!("{:?}", builder);

    let user = get_user_helper(db, token).await?;

    info!("User {:?}", user);

    // let endpoint_str = user.endpoint.clone().expect("Failed to get endpoint");
    // let endpoint = endpoint_str
    //     .parse::<Uri>()
    //     .expect("Failed to parse endpoint URL");


    //FIXME: THis is failing

    let p256dh_str = user.p256dh.clone().expect("Failed to get p256dh").as_str().to_string();
    info!("Encoded p256: {:?}", p256dh_str);
    let bytes = Base64UrlUnpadded::decode_vec(p256dh_str.as_str()).expect("this to be valid base64");
    info!("bytes {:?}", bytes);
    let p2566dh = ES256KeyPair::from_bytes(&bytes).expect("this to be a valid public key");
    info!("test");
    // info!("p2566dh {:?}", p2566dh);

    // let p256dh = BASE64_STANDARD.decode(p256dh_str).expect("Failed to decode p256dh");

    // let p256dh = hex::decode(p256dh_str).expect("Failed to decode p256dh");

    // // Ensure the string is properly padded
    // let padding_needed = p256dh_str.len() % 4;
    // let padding_needed = p256dh_str.len() % 4;
    // if padding_needed > 0 {
    //     for _ in 0..(4 - padding_needed) {
    //         p256dh_str.push('=');
    //     }
    // }
    // let mut p256dh_bytes = [0u8; 65];
    // let p256dh_bytes = decode(&p256dh_str).expect("Failed to decode p256dh");
    // let p256dh = PublicKey::<NistP256>::from_sec1_bytes(&p256dh_bytes).expect("Failed to parse p256dh");
    
    // info!("{:?}", p256dh_str);
    // info!("{:?}", p256dh_str.len() % 4);

    // let p256dh_bytes = decode(&p256dh_str, &mut p256dh_bytes).expect("Failed to decode p256dh");
    // let p256dh = PublicKey::<NistP256>::from_sec1_bytes(&p256dh_bytes.1).expect("Failed to parse p256dh");

    // info!("{:?}", p256dh);

    // let auth_str = row.auth.clone().expect("Failed to get auth");
    // let auth = GenericArray::from_slice(auth_str.as_bytes());

    // let builder = WebPushBuilder::new(endpoint, p256dh, *auth);

    // let https = hyper_util::HttpsConnector::new();
    // let client = Client::builder(TokioExecutor::new()).build(https);

    // let data = json!({
    //     "title": "Time to scout!",
    //     "body": "click me" //todo: make this random funny joke/pun/phrase/quote
    // })
    // .to_string()
    // .into_bytes();

    // let request = builder
    //     .with_vapid(&VAPID_PRIVATE, "")
    //     .build(data)?
    //     .map(axum::body::Body::from);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VapidKey {
    pub public_key: String,
}

#[axum::debug_handler]
pub async fn vapid() -> axum::Json<VapidKey> {
    info!("Getting VAPID key");
    let vapid_key_raw: String = std::env::var("VAPID").expect("VAPID key not set");
    let vapid_key_unparsed = Base64UrlUnpadded::decode_vec(&vapid_key_raw).expect("Failed to decode VAPID key");
    let vapid_key = ES256KeyPair::from_bytes(&vapid_key_unparsed).expect("Failed to parse VAPID key");

    let encoded =
        Base64UrlUnpadded::encode_string(&vapid_key.key_pair().public_key().to_bytes_uncompressed());

    let key = VapidKey {
        public_key: encoded,
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
) -> Result<impl IntoResponse, (StatusCode, String) > {
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
