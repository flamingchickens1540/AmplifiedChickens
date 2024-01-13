use crate::model;
use axum::{
    extract::{Extension, Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::PrivateCookieJar;
use cookie::Cookie;

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl,
    AuthorizationCode, ClientId, ClientSecret,
    RedirectUrl, TokenResponse, TokenUrl,
};


use tracing::{error};

pub fn build_oauth_client(client_id: String, client_secret: String) -> BasicClient {
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");
    let redirect_url = "http://localhost:3007/auth/";

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url.to_string()).unwrap())
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
        Err(_e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Reqwest Error"),
            ))
        }
    };

    let profile: model::User = profile.json::<model::User>().await.unwrap();

    let secs: i64 = token.expires_in().unwrap().as_secs().try_into().unwrap();

    let max_age = chrono::Local::now() + chrono::Duration::seconds(secs);

    let cookie = Cookie::build(("sid", token.access_token().secret().to_owned()))
        .domain(".app.localhost") // change to production url
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(secs));

    if let Err(e) =
        sqlx::query("INSERT INTO users (id, name, avatar_url, coins, scout, coins, points, is_admin) VALUES ($1) ON CONFLICT (id) DO NOTHING")
            .bind(profile.id.clone())
            .bind(profile.name.clone())
            .bind(profile.avatar_url.clone())
            .bind(profile.coins)
            .bind(profile.points)
            .bind(profile.is_admin)
            .execute(&state.db.pool)
            .await
    {
        error!("Error while trying to make account: {e}");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error trying to create account: {e}"),
        ));
    }

    if let Err(e) = sqlx::query("INSERT INTO sessions (user_id, session_id, expires_at) VALUES ((SELECT ID FROM USERS WHERE id = $1 LIMIT 1), $2, $3) ON CONFLICT (user_id) DO UPDATE SET session_id = excluded.session_id, expires_at: excluded.expires_at")
        .bind(profile.id.clone())
        .bind(token.access_token().secret().to_owned())
        .bind(max_age)
        .execute(&state.db.pool)
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

pub async fn admin_auth(
    State(state): State<model::AppState>,
    jar: PrivateCookieJar,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Redirect)> {
    let user: model::User = match get_user(&jar, &state.db).await {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    if !user.is_admin {
        return Err((StatusCode::UNAUTHORIZED, Redirect::to("/protected")));
    }

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
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

    Ok(model::User {
        id: res.id,
        name: res.name,
        avatar_url: res.avatar_url,
        scout: res.scout,
        coins: res.coins,
        points: res.points,
        is_admin: res.is_admin,
    })
}
