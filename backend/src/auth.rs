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
    basic::{BasicClient, BasicTokenType},
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, EmptyExtraTokenFields, RedirectUrl,
    StandardTokenResponse, TokenResponse, TokenType, TokenUrl,
};

use tracing::{error, info};

pub fn build_google_oauth_client(client_id: String, client_secret: String) -> BasicClient {
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");
    let redirect_url = dotenv::var("GOOGLE_REDIRECT_URL")
        .expect("GOOGLE_REDIRECT_URL not set")
        .to_string();

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

pub fn build_slack_oauth_client(client_id: String, client_secret: String) -> BasicClient {
    let auth_url = AuthUrl::new("https://slack.com/oauth/v2/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://slack.com/api/oauth.v2.access".to_string())
        .expect("Invalid token endpoint URL");
    let redirect_url = dotenv::var("SLACK_REDIRECT_URL")
        .expect("SLACK_REDIRECT_URL not set")
        .to_string();

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
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

    let identity = match state
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

    let oauth_data = identity.json::<model::OauthUser>().await.unwrap();

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

    let cookie = insert_user(profile, &state.db, token).await?;

    Ok((jar.add(cookie), Redirect::to("/")))
}

pub async fn slack_callback(
    State(state): State<model::AppState>,
    jar: PrivateCookieJar,
    Query(query): Query<model::AuthRequest>,
    Extension(oauth_client): Extension<BasicClient>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, String)> {
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

    let access_token = token.access_token();

    let identity_response = match state
        .ctx
        .get("https://slack.com/api/users.identity")
        .header("Authorization", format!("Bearer {}", access_token.secret()))
        .send()
        .await
    {
        Ok(res) => res
            .json::<model::SlackIdRes>()
            .await
            .expect("Slack Response did not match expected model"),
        Err(_e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Reqwest Error"),
            ))
        }
    };

    let oauth_data: model::OauthUser = identity_response.user;

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

    let cookie = insert_user(profile, &state.db, token).await?;

    Ok((jar.add(cookie), Redirect::to("/")))
}

async fn insert_user(
    profile: model::User,
    db: &model::Db,
    token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
) -> Result<Cookie<'static>, (StatusCode, String)> {
    let secs: i64 = token.expires_in().unwrap().as_secs().try_into().unwrap();

    let max_age = chrono::Local::now() + chrono::Duration::seconds(secs);

    let cookie = Cookie::build(("sid", token.access_token().secret().to_owned()))
        .domain(".app.localhost") // change to production url
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(secs))
        .build();

    if let Err(e) =
        sqlx::query("INSERT INTO users (id, name, avatar_url, coins, scout, coins, points, is_admin) VALUES ($1) ON CONFLICT (id) DO NOTHING")
            .bind(profile.id.clone())
            .bind(profile.name.clone())
            .bind(profile.is_notify)
    .bind(profile.is_admin)
            .bind(profile.endpoint).bind(profile.p256dh).bind(profile.auth)
            .execute(&db.pool)
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
        .execute(&db.pool)
        .await
    {
        error!("Error while trying to make session: {e}");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error trying to create session: {e}"),
        ));
    }

    info!("Authorized user: {}", profile.name);

    Ok(cookie)
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
        is_notify: res.is_notify,
        is_admin: res.is_admin,
        endpoint: res.endpoint,
        p256dh: res.p256dh,
        auth: res.auth,
    })
}
