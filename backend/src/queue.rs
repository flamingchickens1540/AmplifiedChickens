use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Form, Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, Key};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::error;

use crate::model::{self, AppState, User};

pub async fn get_user(
    State(state): State<AppState>,
    Json(id): Json<String>,
) -> Result<Json<User>, StatusCode> {
    let user: User = match sqlx::query_as("SELECT * FROM \"Users\" WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db.pool)
        .await
    {
        Ok(user) => user,
        Err(e) => return Err(StatusCode::UNAUTHORIZED),
    };

    Ok(Json(user))
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

pub async fn new_match_auto(
    State(state): State<AppState>,
    Form(robots): Form<Vec<String>>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut queue = state.queue.lock().await;
    match queue.new_match_auto_assign(robots, &state.db).await {
        Ok(()) => return Ok(StatusCode::OK),
        Err(err) => match err.0 {
            model::QueueError::EndpointNotSet => {
                return Err(StatusCode::IM_A_TEAPOT); // TODO: Figure out what code to use to indicate that a scout hasn't accepted push notifications
            }
            model::QueueError::QueryFailed => {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        },
    };
}

#[derive(Serialize, Deserialize)]
pub struct ManualMatch {
    robots: Vec<String>,
    scouts: Vec<String>,
}

pub async fn new_match_manual(
    State(state): State<AppState>,
    Form(manual_match): Form<ManualMatch>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut queue = state.queue.lock().await;
    match queue
        .new_match_manual_assign(manual_match.robots, manual_match.scouts, &state.db)
        .await
    {
        Ok(()) => Ok(StatusCode::OK),
        Err(err) => match err.0 {
            model::QueueError::EndpointNotSet => {
                Err(StatusCode::IM_A_TEAPOT) // TODO: Figure out what code to use to indicate that a scout hasn't accepted push notifications
            }
            model::QueueError::QueryFailed => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewEvent {
    event_key: String,
    twitch_link: Option<String>,
}

#[axum::debug_handler]
pub async fn new_event(
    State(state): State<AppState>,
    Form(event): Form<NewEvent>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match sqlx::query("INSERT INTO \"Events\" (event_key, steam_url) VALUES ($1, $2)")
        .bind(event.event_key)
        .bind(event.twitch_link)
        .execute(&state.db.pool)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserPerm {
    admin: bool,
    id: String,
}
#[axum::debug_handler]
pub async fn set_user_permissions(
    State(state): State<AppState>,
    Json(user_perm): Json<UserPerm>,
) -> Result<impl IntoResponse, StatusCode> {
    match sqlx::query("UPDATE \"Users\" SET is_admin = $1 WHERE id = $2")
        .bind(user_perm.admin)
        .bind(user_perm.id)
        .execute(&state.db.pool)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
#[axum::debug_handler]
pub async fn get_finished_matches(
    State(state): State<AppState>,
    Json(event_name): Json<String>,
) -> Result<Json<Vec<String>>, impl IntoResponse> {
    match sqlx::query_as::<_, model::TeamMatch>(
        "SELECT * FROM \"TeamMatches\" WHERE event_key = $1",
    )
    .bind(event_name)
    .fetch_all(&state.db.pool)
    .await
    {
        Ok(matches) => Ok(Json(
            matches
                .into_iter()
                .map(|matches| matches.match_key)
                .collect(),
        )),
        Err(err) => {
            error!("Error fetching all matches: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
#[axum::debug_handler]
pub async fn get_queued_scouts(
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>, impl IntoResponse> {
    let queue = state.queue.lock().await;
    let mut names: Vec<String> = vec![];
    for scout in queue.scouts.iter() {
        names.push(
            match sqlx::query_as::<_, model::User>("SELECT * FROM \"Users\" WHERE id = $1")
                .bind(scout)
                .fetch_one(&state.db.pool)
                .await
            {
                Ok(user) => user.name,
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
        );
    }
    Ok(Json(names))
}

#[axum::debug_handler]
pub async fn get_scouts_and_scouted(
    State(state): State<AppState>,
) -> Result<Json<Vec<(String, i64)>>, (StatusCode, String)> {
    let mut ret: Vec<(String, i64)> = vec![];
    let scouts: Vec<User> = match sqlx::query_as::<_, User>("SELECT * FROM \"Users\"")
        .fetch_all(&state.db.pool)
        .await
    {
        Ok(scouts) => scouts,
        Err(err) => {
            error!("Failed to get scouts: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get scouts".to_string(),
            ));
        }
    };
    for scout in scouts.iter() {
        let name = scout.name.clone();
        let count: i64 = match sqlx::query!("SELECT COUNT(*) FROM \"Users\" WHERE id = $1", name)
            .fetch_one(&state.db.pool)
            .await
        {
            Ok(res) => res.count.unwrap_or(0),
            Err(_) => {
                error!("User in queue not in Db");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to select user from Db".to_string(),
                ));
            }
        };
        ret.push((name, count));
    }
    Ok(Json(ret))
}

pub async fn in_queue(State(state): State<AppState>, Json(id): Json<String>) -> Json<bool> {
    let queue = state.queue.lock().await;
    Json(queue.scouts.contains(&id))
}

#[axum::debug_handler]
pub async fn queue_user(
    State(state): State<AppState>,
    Json(id): Json<String>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut queue = state.queue.lock().await;
    if queue.scouts.contains(&id) {
        error!("Scout already in queue attempted to enter queue");
        return Err((
            StatusCode::BAD_REQUEST,
            "Scout already in queue".to_string(),
        ));
    }

    queue.add_scout_auto_assign(id, &state.db).await;

    Ok((StatusCode::OK, "Success".to_string()))
}
