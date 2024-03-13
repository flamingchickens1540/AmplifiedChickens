use std::convert::Infallible;

use axum::{
    extract::State,
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Response, Sse,
    },
    Form, Json,
};

use futures::future::ok;
use futures_core::{Future, Stream};
use http::HeaderMap;

use serde::{Deserialize, Serialize};

use tokio_stream::wrappers::WatchStream;
use tracing::{error, info};

use crate::model::{self, AllianceColor, AppState, Db, EventState, User};

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

#[derive(Serialize)]
pub struct ScoutResponse {
    team_key: String,
    color: AllianceColor,
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

    let mut robot_queue = state.queue.lock().await;

    let mut team_key = "".to_string();

    let team_color: AllianceColor;

    match headers.get("requested_color") {
        Some(color) => {
            match color.to_str().expect("requested_color was an invalid string") {
                "blue" => {
                    info!("Blue robot requested");
                    team_key = match robot_queue.scout_get_blue() {
                        Some(team) => team,
                        None => return Err((StatusCode::NO_CONTENT, "No robots in blue queue :D".to_string()))
                    };
                    
                    team_color = AllianceColor::Blue;
                },
                "red" => {
                    info!("Red robot requested");
                    team_key = match robot_queue.scout_get_red() {
                        Some(team) => team,
                        None => return Err((StatusCode::NO_CONTENT, "No robots in red queue :D".to_string()))
                    };
                    team_color = AllianceColor::Red;
                },
                "none" => {
                    info!("Agnostic robot requested");
                    match robot_queue.scout_get_robot(access_token.clone()) {
                        Some(team) => {
                            team_key = team.0;
                            team_color = team.1
                        },
                        None => {
                            info!("No robots in queue for scout {}", user.name.clone());
                            return Err((StatusCode::NO_CONTENT, "No robots in queue :D".to_string()))
                        }
                    }
                },
                _ => return Err((StatusCode::BAD_REQUEST, "Invalid requested_color, only red, blue, and none are accepted values".to_string()))
            }
        },
        None => {
            error!("Robot requested without requested_color");
            return Err((StatusCode::BAD_REQUEST, "No request_color provided (try none if you don't care)".to_string()));
        }
    }
        info!("Robot {}, served to user {}", team_key, user.name);
        let res = ScoutResponse {
            team_key: team_key,
            color: team_color,
        };
        Ok(Json(res).into_response())
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewMatchAuto {
    red_teams: Vec<String>,
    blue_teams: Vec<String>,
    match_key: String,
}

#[axum::debug_handler]
pub async fn new_match_auto(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(new_match): Json<NewMatchAuto>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    check_admin_auth(&state.db, headers).await?;
    let mut queue = state.queue.lock().await;
    info!(
        "New Match (Auto Assign): {:?} {:?}",
        new_match.red_teams, new_match.blue_teams
    );
    queue.match_keys.push(new_match.match_key);
    queue
        .new_match_auto_assign(new_match.red_teams, new_match.blue_teams)
        .await;

    let upstream = state.sse_upstream.lock().await;
    match upstream.send(Ok(Event::default().data("match_ready".to_string()))){
        Ok(_) => {info!("Sent match_ready to scout"); Ok(())},
        Err(err) => {
            error!("Error sending new match downstream: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to send new match downstream".to_string(),
            ))
        }
    }
}

#[axum::debug_handler]
pub async fn check_queue(State(state): State<AppState>) -> impl IntoResponse {
    let queue = state.queue.lock().await;

    Json(!queue.red_robots.is_empty() || !queue.blue_robots.is_empty())
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
    red_robots: Vec<String>,
    blue_robots: Vec<String>,
    red_scouts: Vec<String>, // scout names
    blue_scouts: Vec<String>,
    match_key: String,
}

#[axum::debug_handler]
pub async fn new_match_manual(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(manual_match): Json<ManualMatch>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    check_admin_auth(&state.db, headers).await?;

    let mut red_scouts: Vec<String> = vec![];
    let mut blue_scouts: Vec<String> = vec![];

    for scout_name in manual_match.red_scouts {
        let user = match sqlx::query_as::<_, User>("SELECT * FROM \"Users\" WHERE name = $1")
            .bind(scout_name)
            .fetch_one(&state.db.pool)
            .await
        {
            Ok(user) => user.id,
            Err(err) => {
                error!("Assigned user not in DB: {}", err);
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Assigned user does not exist".to_string(),
                ));
            }
        };
        red_scouts.push(user);
    }

    for scout_name in manual_match.blue_scouts {
        let user = match sqlx::query_as::<_, User>("SELECT * FROM \"Users\" WHERE name = $1")
            .bind(scout_name)
            .fetch_one(&state.db.pool)
            .await
        {
            Ok(user) => user.id,
            Err(err) => {
                error!("Assigned user not in DB: {}", err);
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Assigned user does not exist".to_string(),
                ));
            }
        };
        blue_scouts.push(user);
    }

    let mut queue = state.queue.lock().await;
    queue.match_keys.push(manual_match.match_key);
    match queue
        .new_match_manual_assign(
            manual_match.red_robots,
            manual_match.blue_robots,
            red_scouts,
            blue_scouts,
        )
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
) -> Result<Json<Vec<(String, usize)>>, (StatusCode, String)> {
    let mut ret: Vec<(String, usize)> = vec![];
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

    let total: f64 = match sqlx::query(
        "SELECT DISTINCT match_key FROM \"TeamMatches\" WHERE match_key NOT LIKE \'t%\'",
    )
    .fetch_all(&state.db.pool)
    .await
    {
        Ok(res) => res.len() as f64,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to select total team_match count from Db".to_string(),
            ));
        }
    };

    for scout in scouts.iter() {
        let id = scout.id.clone();
        let name = scout.name.clone();
        let name = scout.name.clone();
        let count: f64 = match sqlx::query_as::<_, model::TeamMatch>(
            "SELECT * FROM \"TeamMatches\" WHERE scout_id = $1 AND match_key NOT LIKE \'t%\'",
        )
        .bind(id)
        .fetch_all(&state.db.pool)
        .await
        {
            Ok(res) => {
                //info!("team_matches: {:?}", res);
                res.len() as f64
            }
            Err(e) => {
                error!("Error getting teammatches: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to select user from Db".to_string(),
                ));
            }
        };
        let percent = std::cmp::min((count / total * 100.0) as usize, 100);
        ret.push((name, percent));
    }
    Ok(Json(ret))
}

pub async fn scout_sse_connect(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {

    let upstream = state.sse_upstream.lock().await;

    let downstream = WatchStream::new(upstream.subscribe());

    Sse::new(downstream).keep_alive(KeepAlive::default())
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
                event_key: "2024orsal".to_string(),
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
