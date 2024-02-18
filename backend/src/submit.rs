use crate::model::{AppState, TeamEvent, TeamMatch};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::Json;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use futures::stream::{self, Stream};
use futures::StreamExt;
use std::ops::Deref;
use std::{convert::Infallible, path::PathBuf, time::Duration};
use tokio::sync::watch;
use tokio::time;
use tokio_stream::wrappers::WatchStream;
use tracing::info;

pub async fn admin_sse_connect(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("Admin connected to SSE stream");

    let mut upstreams = state.team_match_upstreams.lock().await;

    let (tx, rx) = watch::channel(Ok(Event::default()));

    let rx: WatchStream<Result<Event, Infallible>> = WatchStream::new(rx);

    upstreams.push(tx);

    Sse::new(rx).keep_alive(KeepAlive::default())
}

pub async fn submit_team_match(
    State(state): State<AppState>,
    Json(form): Json<TeamMatch>,
) -> impl IntoResponse {
    let result = sqlx::query("INSERT INTO \"TeamMatches\" (match_key, team_key, is_fielded, is_leave_start, auto_speaker_succeed, auto_speaker_missed, auto_amp_succeed, auto_amp_missed, auto_piece_succeed, auto_piece_missed, tele_speaker_succeed, tele_speaker_missed, tele_amp_succeed, tele_amp_missed, trap_succeed, trap_missed, stage_enum, skill, notes, is_broke, is_died, scout_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22);").bind(form.match_key).bind(form.team_key).bind(form.is_fielded).bind(form.is_leave_start).bind(form.auto_speaker_succeed).bind(form.auto_speaker_missed).bind(form.auto_amp_succeed).bind(form.auto_amp_missed).bind(form.auto_piece_succeed).bind(form.auto_piece_missed).bind(form.tele_speaker_succeed).bind(form.tele_speaker_missed).bind(form.tele_amp_succeed).bind(form.tele_amp_missed).bind(form.trap_succeed).bind(form.trap_missed).bind(form.stage).bind(form.skill).bind(form.notes).bind(form.is_broke).bind(form.is_died).bind(form.scout_id).execute(&state.db.pool).await;

    match result {
        Ok(_) => return StatusCode::OK,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn submit_pit_data(
    State(state): State<AppState>,
    Json(form): Json<TeamEvent>,
) -> impl IntoResponse {
    let result = sqlx::query("INSERT INTO \"TeamEvents\" (team_key, event_key, width, length, is_short, is_camera, drivetrain_enum, is_ground_intake, is_chute_intake, polish, scout_id) VALUES ($1, $2, $3, $4, $5, $6, $7,$8, $9, $10, $11)").bind(form.team_key).bind(form.event_key).bind(form.width).bind(form.length).bind(form.is_short).bind(form.is_camera).bind(form.drivetrain).bind(form.is_ground_intake).bind(form.is_chute_intake).bind(form.polish).bind(form.scout_id).execute(&state.db.pool).await;

    match result {
        Ok(_) => return StatusCode::OK,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    }
}
