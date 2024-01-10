use crate::model;
use axum::{
    debug_handler,
    extract::{Extension, Host, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Redirect},
    routing::{get, post},
    Router,
};
use axum_extra::{extract::PrivateCookieJar, TypedHeader};
use cookie::Cookie;
use dotenv::{dotenv, var};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, reqwest::http_client, AuthUrl,
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl,
};
use serde_json::json;
use sqlx::PgConnection;
use std::env;
use tracing::{error, info};

pub fn create_api_router() -> Router<model::AppState> {
    let google_client_id =
        ClientId::new(env::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID from .env"));
    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET").expect("Missing GOOGLE_CLIENT_SECRET from .env"),
    );
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");
    let redirect_url = "";
    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url.to_string()).unwrap());

    Router::new()
        .route("/health", get(health_checker_handler))
        .route("/admin_auth", post(google_callback))
        .route("/scout_auth", post(google_callback))
}

pub async fn google_callback(
    State(state): State<model::AppState>,
    jar: PrivateCookieJar,
    Query(query): Query<model::AuthRequest>,
    Extension(oauth_client): Extension<BasicClient>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let token = match oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            error!("An error occurred while exchanging the code: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let profile = match state
        .ctx
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(token.access_token().secret().to_owned())
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Reqwest Error"),
            ))
        }
    };

    let profile = profile.json::<model::User>().await.unwrap();

    let secs: i64 = token.expires_in().unwrap().as_secs().try_into().unwrap();

    let max_age = chrono::Local::now() + chrono::Duration::seconds(secs);

    let cookie = Cookie::build(("sid", token.access_token().secret().to_owned()))
        .domain(".app.localhost") // change to production url
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(secs));

    if let Err(e) =
        query("INSERT INTO users (id, pw_hash, coins) VALUES ($1) ON CONFLICT (id) DO NOTHING")
            .bind(profile.id.clone())
            .bind(profile.pw_hash.clone())
            .bind(profile.coins.clone())
            .execute(&state.db)
            .await
    {
        error!("Error while trying to make account: {e}");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error trying to create account: {e}"),
        ));
    }

    if let Err(e) = query("INSERT INTO sessions (user_id, session_id, expires_at) VALUES ((SELECT ID FROM USERS WHERE id = $1 LIMIT 1), $2, $3) ON CONFLICT (user_id) DO UPDATE SET session_id = excluded.session_id, expires_at: excluded.expires_at")
        .bind(profile.id.clone())
        .bind(token.access_token().secret().to_owned())
        .bind(max_age)
        .execute(&state.db)
        .await
    {
        error!("Error while trying to make session: {e}");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error trying to create session: {e}"),
        ));
    }

    Ok((jar.add(cookie), Redirect::to("/")))
}

fn build_oauth_client(client_id: String, client_secret: String) -> BasicClient {
    // TODO: Replace with production url
    let frontend_host = var("FRONT_HOST").expect("fronthost not set");
    let frontend_port = var("FRONT_PORT").expect("frontport not set");
    let redirect_url = format!("https://{frontend_host}:{frontend_port}/");
    println!("redirect_url: {redirect_url}");

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

// Standard auth :)

// FIXME: Use same password hashing funtion on both to check
async fn admin_auth_handler(Json(auth_data): Json<model::User>) -> impl IntoResponse {
    dotenv().ok();
    let admin_password = var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD is not set");
    let response;
    if admin_password == auth_data.password {
        let session: String = String::from("");
        response = json!({
            "code": 200,
            "authed": true,
            "session": session
        });
    } else {
        response = json!({
            "code": 200,
            "authed": false
        });
    }
    return Json(response);
}

async fn scout_auth_handler(Json(auth_data): Json<model::User>) -> impl IntoResponse {
    dotenv().ok();
    let scout_password = var("SCOUT_PASSWORD").expect("SCOUT_PASSWORD is not set");
    let response;
    if scout_password == auth_data.password {
        let session: String = String::from("");
        response = json!({
            "authed": true,
            "session": session
        });
    } else {
        response = json!({
            "authed": false
        });
    }
    return Json(response);
}

async fn health_checker_handler() -> StatusCode {
    StatusCode::OK
}
