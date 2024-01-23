use crate::model;
use axum::{
    extract::{Extension, Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::PrivateCookieJar;
use cookie::Cookie;
use serde_json::Value;

use tracing::{error, info};

pub async fn slack_callback(
    State(state): State<model::AppState>,
    jar: PrivateCookieJar,
    Query(query): Query<model::AuthRequest>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, String)> {
    info!("THIS");
    let client_secret = dotenv::var("SLACK_CLIENT_SECRET").unwrap();
    let client_id = dotenv::var("SLACK_CLIENT_ID").unwrap();
    let redirect_url = dotenv::var("SLACK_REDIRECT_URL").unwrap();
    info!("{}", redirect_url);
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

    let access_token = token_res.get("access_token").unwrap().to_string();
    info!("Access Token: {}", access_token);

    let identity_response = match state
        .ctx
        .get("https://slack.com/api/openid.connect.userInfo")
        .header("content_type", "application/x-www-form-urlencoded")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
    {
        Ok(res) => res.json::<serde_json::Value>().await,
        Err(e) => {
            return Err::<(PrivateCookieJar, Redirect), (axum::http::StatusCode, std::string::String)>(
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Reqwest Error: {e}"),
                ),
            )
        }
    };

    match identity_response {
        Ok(res) => match res {
            Value::Object(obj) => {
                info!("{:?}", obj); // happening "invalid_auth"
                if obj.get("ok").unwrap().as_bool().unwrap() {
                    let user = obj.get("user").unwrap();
                    let oauth_data: model::OauthUser = model::OauthUser {
                        id: user.get("id").unwrap().to_string(),
                        name: user.get("name").unwrap().to_string(),
                    };

                    // TODO: Check if it's valid to assume they only log in once ever, so we can set scouting stats to default
                    let profile = model::User::new(
                        oauth_data.id,
                        oauth_data.name,
                        false,
                        false,
                        None,
                        None,
                        None,
                    );

                    let cookie = insert_user(
                        profile,
                        &state.db,
                        (
                            obj.get("expires_in").unwrap().as_i64().unwrap(),
                            access_token,
                        ),
                    )
                    .await?;
                    Ok((jar.add(cookie), Redirect::to("/")))
                } else {
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Identity request returned not ok".to_string(),
                    ))
                }
            }
            _ => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Identity request return unexpected json".to_string(),
            )),
        },
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Incorrectly Authed, Identity response failed: {}", e),
        )),
    }
}

pub async fn fake_callback(
    State(state): State<model::AppState>,
    jar: PrivateCookieJar,
    Query(query): Query<model::AuthRequest>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, String)> {
    let client_secret = dotenv::var("SLACK_CLIENT_SECRET").unwrap();
    let client_id = dotenv::var("SLACK_CLIENT_ID").unwrap();
    let redirect_url = dotenv::var("SLACK_REDIRECT_URL").unwrap();
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

    let access_token = token_res.get("access_token").unwrap().to_string();
    info!("Access Token: {}", access_token);

    let id =
        sqlx::query("SELECT user_id FROM users ORDER BY id ASC LIMIT 1").fetch_one(&state.db.pool);

    let profile = model::User::new(
        "".to_string(),
        "TestUser".to_string(),
        false,
        false,
        None,
        None,
        None,
    );

    let cookie = insert_user(profile, &state.db, (300000, access_token)).await?;
    Ok((jar.add(cookie), Redirect::to("/")))
}

pub async fn user_auth(
    State(state): State<model::AppState>,
    jar: PrivateCookieJar,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Redirect)> {
    let user: model::User = match get_user(&jar, &state.db).await {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

// token_res: (expires_in, access_token)
async fn insert_user(
    profile: model::User,
    db: &model::Db,
    token_res: (i64, String),
) -> Result<Cookie<'static>, (StatusCode, String)> {
    let secs: i64 = token_res.0 / 100;

    let max_age = cookie::time::Instant::now() + cookie::time::Duration::seconds(secs);

    let cookie = Cookie::build(("sid", token_res.1))
        .domain(".app.localhost")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(secs))
        .finish();

    if let Err(e) =
        sqlx::query("INSERT INTO users (id, name, is_notify, is_admin, endpoin, p256dh, auth) VALUES ($1) ON CONFLICT (id) DO NOTHING")
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
        error!("Error while trying to make account: {e}");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error trying to create account: {e}"),
        ));
    };
    Ok(cookie)
}

async fn get_user(
    jar: &PrivateCookieJar,
    db: &model::Db,
) -> Result<model::User, (StatusCode, Redirect)> {
    let Some(cookie) = jar.get("sid").map(|cookie| cookie.value().to_owned()) else {
        return Err((StatusCode::UNAUTHORIZED, Redirect::to("/")));
    };
    let res = match sqlx::query_as::<_, model::User>(
        "SELECT 
        users
        FROM sessions 
        LEFT JOIN USERS ON sessions.user_id = users.id
        WHERE sessions.session_id = $1 
        LIMIT 1",
    )
    .bind(cookie)
    .fetch_one(&(db.pool))
    .await
    {
        Ok(res) => res,
        Err(_e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Redirect::to("/"))),
    };

    Ok(model::User { ..res })
}
