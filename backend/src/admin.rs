use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, Key};
use sqlx::PgPool;

use crate::model::{AppState, User};

pub async fn get_user(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<serde_json::Value, StatusCode> {
    let access_token = match jar.get("sid") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let user: User = match sqlx::query_as("SELECT * FROM \"Users\" WHERE auth EQUALS $1")
        .bind(access_token)
        .fetch_one(&state.db.pool)
        .await
    {
        Ok(user) => user,
        Err(e) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(serde_json::to_value(user).unwrap())
}

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<(StatusCode, Vec<User>), (StatusCode, String)> {
    // TODO: Make sure an error isn't returned if there aren't any users logged in
    match sqlx::query_as("SELECT * FROM \"Users\"")
        .fetch_all(&state.db.pool)
        .await
    {
        Ok(users) => Ok((StatusCode::OK, users)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch users: {err}").to_string(),
        )),
    }
}

//pub async fn queue_match(State(state): State<AppState>, Form(robots): Form<Vec<String>>) {
//}
