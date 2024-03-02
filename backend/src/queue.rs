use std::convert::Infallible;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form, Json,
};

use http::HeaderMap;

use serde::{Deserialize, Serialize};

use tracing::{error, info};

use crate::model::{self, AppState, Db, EventState, User};

pub async fn get_user_helper(db: &Db, token: String) -> Result<Json<User>, (StatusCode, String)> {
    let user: User = match sqlx::query_as("SELECT * FROM \"Users\" WHERE access_token = $1")
        .bind(token)
        .fetch_one(&db.pool)
        .await
    {
        Ok(user) => user,
        Err(_e) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Unauthorzed user; invalid access_token".to_string(),
            ))
        }
    };

    Ok(Json(user))
}

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    // TODO: Make sure an error isn't returned if there aren't any users logged in
    match sqlx::query_as("SELECT * FROM \"Users\"")
        .fetch_all(&state.db.pool)
        .await
    {
        Ok(users) => Ok(Json(users)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch users: {err}").to_string(),
        )),
    }
}

#[axum::debug_handler]
pub async fn scout_request_team(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, (StatusCode, String)> {
    let access_token = match headers.get("x-access-token") {
        Some(token) => token
            .to_str()
            .expect("access_token was an invalid string")
            .to_string(),
        None => {
            error!("Robot requested without access_token");
            return Err((
                StatusCode::UNAUTHORIZED,
                "Unauthorzed, no access_token provided".to_string(),
            ));
        }
    };
    let user = get_user_helper(&state.db, access_token.clone()).await?;


    info!("{} requested team", user.name);

    let mut robot_queue = state.queue.lock().await;
    match robot_queue.scout_get_robot(access_token.clone()) {
        Some(team) => {
            info!(
                "Robot {}, served to user {}", 
                team, user.name 
            );
            Ok(Json(team).into_response())
        }
        None => {
            info!("No robots in queue :D");
            Err((StatusCode::NO_CONTENT, "No robots in queue :D".to_string()))
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewMatchAuto {
     teams: Vec<String>,
     match_key: String
}

#[axum::debug_handler]
pub async fn new_match_auto(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(new_match): Json<NewMatchAuto>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    check_admin_auth(&state.db, headers).await?;
    let mut queue = state.queue.lock().await;
    info!("New Match (Auto Assign): {:?}", new_match.teams);
    queue.match_keys.push(new_match.match_key);
    queue.new_match_auto_assign(new_match.teams).await;
    Ok(())
}

pub async fn check_admin_auth(db: &Db, headers: HeaderMap) -> Result<(), (StatusCode, String)> {
    let code: String = match headers.get("x-access-token") {
        Some(code) => code
            .to_str()
            .expect("Header was not valid UTF-8")
            .to_string(),
        None => return Err((StatusCode::UNAUTHORIZED, "No access code given".to_string())),
    };

    let user = match sqlx::query_as::<_, User>("SELECT * from \"Users\" WHERE access_token = $1")
        .bind(code)
        .fetch_one(&db.pool)
        .await
    {
        Ok(user) => user,
        Err(err) => {
            error!("{}", err);
            return Err((StatusCode::UNAUTHORIZED, "User not in Db".to_string()));
        }
    };

    if !user.is_admin {
        return Err((StatusCode::UNAUTHORIZED, "User is not admin".to_string()));
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct ManualMatch {
    robots: Vec<String>,
    scouts: Vec<String>,
    match_key: String
}

#[axum::debug_handler]
pub async fn new_match_manual(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(manual_match): Json<ManualMatch>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    check_admin_auth(&state.db, headers).await?;

    let mut queue = state.queue.lock().await;
queue.match_keys.push(manual_match.match_key);
    match queue
        .new_match_manual_assign(manual_match.robots, manual_match.scouts)
        .await
    {
        Ok(()) => Ok(()),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

pub async fn get_current_match(State(state): State<AppState>) -> Json<String> {
    let manager = state.queue.lock().await;

    if manager.match_keys.len() == 0 {
        return Json("2024orore_qm1".to_string());
    }

    Json(manager.match_keys[manager.match_keys.len() - 1].clone())
}

#[derive(Serialize, Deserialize)]
pub struct NewEvent {
    code: String,
    event_key: String,
    twitch_link: Option<String>,
}

#[axum::debug_handler]
pub async fn new_event(
    State(state): State<AppState>,
    Form(event): Form<NewEvent>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user: User = match get_user_helper(&state.db, event.code).await {
        Ok(user) => user.0,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "User not in DB".to_string())),
    };
    if !user.is_admin {
        return Err((StatusCode::UNAUTHORIZED, "User is not admin".to_string()));
    }
    match sqlx::query("INSERT INTO \"Events\" (event_key, steam_url) VALUES ($1, $2)")
        .bind(event.event_key)
        .bind(event.twitch_link)
        .execute(&state.db.pool)
        .await
    {
        Ok(_) => Ok(()),
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
    // Send through sse here
    match sqlx::query("UPDATE \"Users\" SET is_admin = $1 WHERE access_token = $2")
        .bind(user_perm.admin)
        .bind(user_perm.id)
        .execute(&state.db.pool)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(_err) => Err(StatusCode::INTERNAL_SERVER_ERROR),
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
pub async fn get_scouts_and_scouted(
    State(state): State<AppState>,
) -> Result<Json<(Vec<String>, Vec<i64>)>, (StatusCode, String)> {
    let mut ret: (Vec<String>, Vec<i64>) = (vec![], vec![]);
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

    let total: i64 = match sqlx::query!("SELECT COUNT(*) FROM \"TeamMatches\"")
        .fetch_one(&state.db.pool)
        .await
    {
        Ok(res) => res.count.unwrap_or(0),
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to select total user count from Db".to_string(),
            ));
        }
    };

    for scout in scouts.iter() {
        let id = scout.id.clone();
        let name = scout.name.clone();
        // TODO: Figure out how do COUNT commands without macros, because macros check for a db connection and are annoying for dev
        let count: i64 = match sqlx::query!(
            "SELECT COUNT(*) FROM \"TeamMatches\" WHERE scout_id = $1",
            id
        )
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
        ret.0.push(name);
        ret.1.push(count / total);
    }
    Ok(Json(ret))
}

#[axum::debug_handler]
pub async fn get_unpitscouted_teams(
    State(state): State<AppState>,
) -> Result<Json<Vec<model::Team>>, Infallible> {
    let current_event = sqlx::query_as::<_, model::EventState>("SELECT* FROM \"EventState\"")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or_else(|_| -> EventState {
            error!("Failed to get eventstate, falling back on default");
            model::EventState {
                event_key: "2024orore".to_string(),
                last_match: None,
                next_match: None,
            }
        });
// FIXME: Every TeamEvent has to be loaded with at least width = 0 (or null or smth) before the
// event for this function to work. edit as needed if jack's busy and that's too much work
    Ok(Json(
        sqlx::query_as::<_, model::Team>(
            "SELECT * FROM \"Teams\" WHERE event_key = $1 AND width = 0",
        )
        .bind(current_event.event_key)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or(vec![]),
    ))
}
