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
