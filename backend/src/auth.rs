use crate::model::{self, AppState};
use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use http::header::{LOCATION, SET_COOKIE};

use jsonwebtoken::*;

use serde::Deserialize;
use tracing::{error, info};

#[axum::debug_handler]
pub async fn slack_callback(
    State(state): State<model::AppState>,
    Query(query): Query<model::AuthRequest>,
) -> Result<axum::http::Response<String>, (axum::http::StatusCode, String)> {
    let client_secret = dotenv::var("SLACK_CLIENT_SECRET").unwrap();
    let client_id = dotenv::var("SLACK_CLIENT_ID").unwrap();
    let redirect_url = dotenv::var("SLACK_REDIRECT_URL").unwrap();
    let _signing_secret = dotenv::var("SLACK_SIGNING_SECRET").unwrap();
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
    let _max_age = chrono::Local::now().naive_local() + chrono::Duration::seconds(exp);

    let profile = model::User::new(
        sub.clone(),
        name.to_string(),
        false,
        false,
        None,
        None,
        None,
        access_token.clone(),
    );

    let current_event_key =
        match sqlx::query_as::<_, model::EventState>("SELECT * FROM \"EventState\"")
            .fetch_one(&state.db.pool)
            .await
        {
            Ok(state) => state.event_key,
            Err(_) => {
                error!("Failed to get current event key");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to get current event key".to_string(),
                ));
            }
        };

    let _id = insert_user(profile.clone(), state.db).await?;

    Ok(axum::http::Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(LOCATION, "http://localhost:3020/app/home")
        .header(
            SET_COOKIE,
            format!(
                "access_token={}; Path=/; SameSite=None; Secure",
                access_token
            ),
        )
        .header(
            SET_COOKIE,
            format!("scout_name={}; Path=/; SameSite=None; Secure", name),
        )
        .header(
            SET_COOKIE,
            format!("scout_id={}; Path=/; SameSite=None; Secure", sub),
        )
        .header(
            SET_COOKIE,
            format!(
                "current_event_key={}; Path=/; SameSite=None; Secure",
                current_event_key
            ),
        )
        .body("Redirecting...".to_string())
        .unwrap())

    //let mut response = axum::http::Response::new(axum::http::StatusCode::OK);
    //response.headers_mut().insert(
    //   "Set-Cookie",
    //   axum::http::HeaderValue::from_str(&format!("user_data={}; Path=/; HttpOnly", profile.id))
    //       .unwrap(),
    //);
}

pub async fn logout(
    State(state): State<model::AppState>,
    Json(access_token): Json<String>,
) -> Result<impl IntoResponse, (String, StatusCode)> {
    if let Err(err) = sqlx::query("DELETE FROM \"Users\" WHERE acess_token = $1 LIMIT 1")
        .bind(access_token)
        .execute(&state.db.pool)
        .await
    {
        error!("Failed to remove user from db: {err}");
        return Err((
            "Failed to remove user form db".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let response = axum::http::Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(LOCATION, "http://localhost:3020/app/home") // TODO: Change to /app/home
        .header(
            SET_COOKIE,
            format!(
                "access_token=deleted; Path=/; SameSite=None; expires=Thu, 01 Jan 1970 00:00:00 GMT"
            ),
        )
        .header(
            SET_COOKIE,
            format!(
                "scout_name=deleted; Path=/; SameSite=None; expires=Thu, 01 Jan 1970 00:00:00 GMT"
            ),
        )
        .header(
            SET_COOKIE,
            format!(
                "scout_id=deleted; Path=/; SameSite=None; expires=Thu, 01 Jan 1970 00:00:00 GMT"
            ),
        )
        .header(
            SET_COOKIE,
            format!(
                "current_event_key=deleted; Path=/; SameSite=None; expires=Thu, 01 Jan 1970 00:00:00 GMT"
            ),
        )
        .body("Redirecting...".to_string())
        .unwrap();

    Ok(response)
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    access_token: String,
    is_admin: bool,
}

//
pub async fn check_auth(
    State(state): State<AppState>,
    Json(req): Json<AuthRequest>,
) -> Result<(), (StatusCode, String)> {
    info!("Check auth");
    info!("Access Token: {}", req.access_token);

    let users = sqlx::query_as::<_, model::User>("SELECT * FROM \"Users\"")
        .fetch_all(&state.db.pool)
        .await
        .expect("Some users to exist");

    info!("All Users: {:?}", users);

    let user: model::User = match sqlx::query_as("SELECT * FROM \"Users\" WHERE access_token = $1")
        .bind(format!("\"{}\"", req.access_token))
        .fetch_optional(&state.db.pool)
        .await
    {
        Ok(user) => match user {
            Some(val) => val,
            None => {
                error!("User is not in DB");
                return Err((
                    StatusCode::UNAUTHORIZED,
                    "User does not exist in DB".to_string(),
                ));
            }
        },
        Err(err) => {
            error!("ERROR LOOKING USER UP FOR AUTH: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to lookup user".to_string(),
            ));
        }
    };

    if !user.is_admin && req.is_admin {
        error!("Not Admin User attepted to access admin route");
        return Err((StatusCode::UNAUTHORIZED, "User is not an admin".to_string()));
    }

    Ok(())
}

// token_res: (expires_in, access_token)
async fn insert_user(profile: model::User, db: model::Db) -> Result<(), (StatusCode, String)> {
    //let max_age: i64 = chrono::Local::now().timestamp_millis() * 100 + secs;
    //cookie.set_max_age(max_age);
    info!("New user\n{:?}", profile);

    if let Err(e) =
        sqlx::query("INSERT INTO \"Users\" (id, name, is_notify, is_admin, endpoint, p256dh, auth, access_token) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT(id) DO NOTHING")
            .bind(profile.id.clone())
            .bind(profile.name.clone())
            .bind(profile.is_notify)
            .bind(profile.is_admin)
            .bind(profile.endpoint)
            .bind(profile.p256dh)
            .bind(profile.auth)
            .bind(profile.access_token)
            .execute(&db.pool)
            .await
    {
        error!("Error trying to create account: {e}");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error trying to create account: {e}"),
        ));
    };
    Ok(())
}

async fn get_user(access_token: String, db: &model::Db) -> Result<model::User, Redirect> {
    //let cookie = jar.get("sid");
    //info!("Access token: {:?}", cookie);
    //let Some(cookie) = jar.get("sid").map(|cookie| cookie.value().to_owned()) else {
    //    error!("Unauthorized user attempted to query a protected endpoint");
    //   return Err((StatusCode::UNAUTHORIZED, Redirect::to("/")));
    //};

    let res = match sqlx::query_as::<_, model::User>("SELECT * FROM \"Users\" WHERE auth = $1")
        .bind(access_token)
        .fetch_one(&(db.pool))
        .await
    {
        Ok(res) => res,
        Err(e) => {
            error!("{}", e);
            return Err(Redirect::to("http://localhost:3020/"));
        }
    };

    Ok(model::User { ..res })
}
