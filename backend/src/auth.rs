use std::{collections::BTreeMap, ops::AddAssign, os::unix::fs::chroot};

use crate::model;
use axum::{
    extract::{Extension, Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use dotenv::dotenv;
use jsonwebtoken::*;
use serde_json::Value;

use tracing::{error, info};

pub async fn slack_callback(
    State(state): State<model::AppState>,
    jar: CookieJar,
    Query(query): Query<model::AuthRequest>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, String)> {
    let client_secret = dotenv::var("SLACK_CLIENT_SECRET").unwrap();
    let client_id = dotenv::var("SLACK_CLIENT_ID").unwrap();
    let redirect_url = dotenv::var("SLACK_REDIRECT_URL").unwrap();
    let signing_secret = dotenv::var("SLACK_SIGNING_SECRET").unwrap();
    info!("Redirect URL: {}", redirect_url);
    //let nonce = "test_nonce";

    let token_res: serde_json::Value = state
        .ctx
        .post("https://slack.com/api/openid.connect.token")
        .query(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", query.code),
            ("redirect_uri", redirect_url),
            ("grant_type", "authorization_code".to_string()),
        ])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    info!("Token Response: {:?}", token_res);

    if !token_res.get("ok").unwrap().as_bool().unwrap() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication failed".to_string(),
        ));
    }

    let access_token = token_res.get("access_token").unwrap().to_string();
    let id_token = token_res.get("id_token").unwrap().as_str().unwrap();

    let key = DecodingKey::from_secret(&[]);
    let mut validation: Validation = Validation::new(Algorithm::HS256);
    validation.insecure_disable_signature_validation();
    validation.validate_exp = false;
    validation.validate_aud = false;

    let data: TokenData<serde_json::Value> =
        decode::<serde_json::Value>(id_token, &key, &validation).unwrap();

    //let decoding_key = DecodingKey::from_secret(signing_secret.as_bytes());
    //let decoded_token = verify_signature
    //let decoded_token =
    //    decode::<model::SlackClaims>(&id_token, &decoding_key, &Validation::default())
    //        .expect("Failed to decode token");

    let name = data.claims.get("name").unwrap().as_str().unwrap();
    let exp = data.claims.get("exp").unwrap().as_i64().unwrap();
    let sub = data
        .claims
        .get("sub")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    info!("Name: {}", name);
    info!("Exp: {}", exp);
    info!("Sub: {}", sub);
    info!("{:?}", data.claims);
    info!("Access Token: {}", access_token);
    let max_age = chrono::Local::now().naive_local() + chrono::Duration::seconds(exp);

    let profile = model::User::new(sub, name.to_string(), false, false, None, None, None);

    let cookie = insert_user(profile, state.db, (exp, access_token)).await?;
    Ok((jar.add(cookie), Redirect::to("http://localhost:5173/")))
}

pub async fn logout(
    State(state): State<model::AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, (String, StatusCode)> {
    let id = jar.get("sid").unwrap();

    if let Err(err) = sqlx::query("DELETE FROM \"Users\" WHERE id EQUALS $1 LIMIT 1")
        .bind(id.value())
        .execute(&state.db.pool)
        .await
    {
        error!("Failed to remove user from db: {err}");
        return Err((
            "Failed to remove user form db".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    Ok((jar.remove("sid"), StatusCode::OK))
}

pub async fn user_auth(
    State(state): State<model::AppState>,
    Query(access_token): Query<String>,
) -> Result<(StatusCode, model::User), (StatusCode, Redirect)> {
    get_user(access_token, &state.db).await
}

pub async fn admin_auth(
    State(state): State<model::AppState>,
    Query(access_token): Query<String>,
) -> Result<(StatusCode, model::User), (StatusCode, Redirect)> {
    let res = get_user(access_token, &state.db).await?;

    if !res.1.is_admin {
        return Err((
            StatusCode::UNAUTHORIZED,
            Redirect::to("http://localhost:5173/"),
        ));
    }

    Ok(res)
}

// token_res: (expires_in, access_token)
async fn insert_user(
    profile: model::User,
    db: model::Db,
    token_res: (i64, String),
) -> Result<Cookie<'static>, (StatusCode, String)> {
    let secs: i64 = token_res.0 / 100;

    let max_age = chrono::Local::now().timestamp_millis() * 100 + secs;
    let cookie = Cookie::new("sid", token_res.1);

    if let Err(e) =
        sqlx::query("INSERT INTO \"Users\" (id, name, is_notify, is_admin, endpoint, p256dh, auth) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (id) DO NOTHING")
            .bind(profile.id.clone())
            .bind(profile.name.clone())
            .bind(profile.is_notify)
            .bind(profile.is_admin)
            .bind(profile.endpoint)
            .bind(profile.p256dh)
            .bind(profile.auth)
            .execute(&db.pool)
            .await
    {
        error!("Error trying to create account: {e}");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error trying to create account: {e}"),
        ));
    };
    Ok(cookie)
}

async fn get_user(
    access_token: String,
    db: &model::Db,
) -> Result<(StatusCode, model::User), (StatusCode, Redirect)> {
    //let cookie = jar.get("sid");
    //info!("Access token: {:?}", cookie);
    //let Some(cookie) = jar.get("sid").map(|cookie| cookie.value().to_owned()) else {
    //    error!("Unauthorized user attempted to query a protected endpoint");
    //   return Err((StatusCode::UNAUTHORIZED, Redirect::to("/")));
    //};

    let res = match sqlx::query_as::<_, model::User>(
        "SELECT * FROM \"Users\" WHERE access_token EQUALS $1 LIMIT 1",
    )
    .bind(access_token)
    .fetch_one(&(db.pool))
    .await
    {
        Ok(res) => res,
        Err(e) => {
            error!("{}", e);
            return Err((
                StatusCode::UNAUTHORIZED,
                Redirect::to("http://localhost:5173/"),
            ));
        }
    };

    Ok((StatusCode::OK, model::User { ..res }))
}
