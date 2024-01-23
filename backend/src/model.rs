use axum::extract::FromRef;
use cookie::Key;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::Uuid, Pool, Postgres};

use reqwest::Client as ReqwestClient;

#[derive(Debug, Clone)]
pub struct Db {
    pub pool: Pool<Postgres>,
}

impl Db {
    pub async fn new(db_url: String) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new().connect(&db_url).await?;

        let migrator = sqlx::migrate!();
        migrator.run(&pool).await?;

        Ok(Db { pool })
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub ctx: ReqwestClient,
    pub key: Key,
}
// implementing FromRef is required here so we can extract substate in Axum
// read more here: https://docs.rs/axum/latest/axum/extract/trait.FromRef.html
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[derive(Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub is_notify: bool,
    pub is_admin: bool,
    pub endpoint: Option<String>,
    pub p256dh: Option<String>,
    pub auth: Option<String>,
}

impl User {
    pub fn new(
        id: String,
        name: String,
        is_notify: bool,
        is_admin: bool,
        endpoint: Option<String>,
        p256dh: Option<String>,
        auth: Option<String>,
    ) -> Self {
        User {
            id,
            name,
            is_notify,
            is_admin,
            endpoint,
            p256dh,
            auth,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct OauthUser {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SlackIdRes {
    pub ok: bool,
    pub user: OauthUser,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub code: String,
}

// Database model

#[derive(Debug, Deserialize, Clone)]
pub struct ScoutEventTeam {
    pub event_key: String,
    pub team_key: String,
    pub scout_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct TeamMatch {
    pub id: Option<i32>,
    pub match_key: String,
    pub team_key: String,
    pub is_fielded: bool,
    pub is_leave_start: bool,
    pub auto_speaker_succeed: i16,
    pub auto_speaker_missed: i16,
    pub auto_amp_succeed: i16,
    pub auto_amp_missed: i16,
    pub auto_piece_succeed: i16,
    pub auto_piece_missed: i16,
    pub tele_speaker_succeed: i16,
    pub tele_speaker_missed: i16,
    pub tele_amp_succeed: i16,
    pub tele_amp_missed: i16,
    pub trap_succeed: bool,
    pub trap_missed: bool,
    pub stage: Stage,
    pub skill: Skill,
    pub notes: String,
    pub is_broke: bool,
    pub is_died: bool,
    pub scout_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Team {
    team_key: String,
    nickname: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Match {
    match_key: String,
    event_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Video {
    match_key: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct TBAMatch {
    match_key: String,
    event_key: String,
    time: chrono::NaiveTime,
    red_1: String,
    red_2: String,
    red_3: String,
    blue_1: String,
    blue_2: String,
    blue_3: String,
    video: i64,
}

// Pitscouting
#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct TeamEvent {
    id: i64,
    team_key: String,
    event_key: String,
    width: i16,
    length: i16,
    is_short: bool,
    is_camera: bool,
    drivetrain: DriveTrain,
    is_ground_intake: bool,
    is_chute_intake: bool,
    polish: Polish,
    scout_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Image {
    id: i64,
    event_key: String,
    team_key: String,
    url: String,
    scout_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct EventState {
    event_key: String,
    next_match: String,
    last_match: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Stage {
    OnState,
    Park,
    NotAttempted,
    Failed,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Skill {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum DriveTrain {
    Swerve,
    Tank,
    Other,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Polish {
    One,
    Two,
    Three,
    Four,
    Five,
}
