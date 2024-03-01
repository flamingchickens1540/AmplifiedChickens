use axum::response::sse::Event;
use std::convert::Infallible;
use std::{collections::HashMap};
use std::sync::Arc;
use tokio::sync::watch::Sender;
use tokio::sync::Mutex;
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

#[derive(Debug, Clone)]
pub enum CurrentMatchType {
    Manual,
    Auto,
}

#[derive(Debug, Clone)]
pub struct Db {
    pub pool: Pool<Postgres>,
}

impl Db {
    pub async fn new(db_url: String) -> Result<Self, sqlx::Error> {
        let pool: Pool<Postgres> = PgPoolOptions::new().connect(&db_url).await?;

        Ok(Db { pool })
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub ctx: ReqwestClient,
    pub queue: Arc<Mutex<RoboQueue>>,
    pub sse_upstream: Arc<Mutex<Sender<Result<Event, Infallible>>>>,
}

#[derive(Debug, Clone)]
pub struct RoboQueue {
    pub match_keys: Vec<String>,
    // This is only for manual assignment
    pub assigned: HashMap<String, String>, // access_token: team_key
    pub robots: Vec<String>,
    pub curr_match_type: CurrentMatchType,
}

// We're assuming that all three red and all three blue is the order of the robots
impl RoboQueue {
    /// Takes in a list of robots,
    /// Assigns each robot to one of the queued scouts, puts all the remaining robots in the robots queue
    pub async fn new_match_auto_assign(&mut self, mut robots: Vec<String>) {
        self.robots.append(&mut robots);
        self.curr_match_type = CurrentMatchType::Manual;
    }

    // Manual assign
    pub async fn new_match_manual_assign(
        &mut self,
        robots: Vec<String>,
        scouts: Vec<String>,
    ) -> Result<(), String> {
        assert_eq!(robots.len(), scouts.len());
        for (i, _) in robots.iter().enumerate() {
            info!("Robot pushed to scout");
            if self.assigned.contains_key(&scouts[i].clone()) {
                return Err("Scout already manually assigned".to_string());
            }
            self.assigned.insert(scouts[i].clone(), robots[i].clone());
            info!(
                "Scout {} manually assigned to team {}",
                scouts[i], robots[i]
            );
        }
        self.curr_match_type = CurrentMatchType::Manual;
        Ok(())
    }

    pub fn scout_get_robot(&mut self, scout: String) -> Option<String> {
        match self.assigned.get(&scout) {
            Some(scout) => Some(scout.clone()),
            None => self.robots.pop(),
        }
    }
}

pub enum QueueError {
    EndpointNotSet,
    QueryFailed,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub is_notify: bool,
    pub is_admin: bool,
    pub endpoint: Option<String>,
    pub p256dh: Option<String>,
    pub auth: Option<String>,
    pub access_token: String,
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
        access_token: String,
    ) -> Self {
        User {
            id,
            name,
            is_notify,
            is_admin,
            endpoint,
            p256dh,
            auth,
            access_token,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SlackClaims {
    pub exp: i64,
    pub name: String,
}

// Database model

#[derive(Debug, Deserialize, Clone)]
pub struct ScoutEventTeam {
    pub event_key: String,
    pub team_key: String,
    pub scout_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow, Default)]
pub struct TeamMatch {
    pub id: i64,
    pub match_key: String,
    pub team_key: String,
    pub is_fielded: bool,
    pub is_leave_start: bool,
    pub auto_speaker_succeed: i16, //
    pub auto_speaker_missed: i16, //
    pub auto_amp_succeed: i16, //
    pub auto_amp_missed: i16, //
    pub auto_piece_succeed: i16, //
    pub auto_piece_missed: i16, //
    pub tele_speaker_succeed: i16, //
    pub tele_speaker_missed: i16, //
    pub tele_amp_succeed: i16, // 
    pub tele_amp_missed: i16, //
    pub trap_succeed: i16, // here's the issue 
    pub trap_missed: i16, //
    pub stage: String,
    pub skill: i16,
    pub notes: String,
    pub is_broke: bool,
    pub is_died: bool,
    pub scout_id: String,
    pub location: String 
}

// For use later when we fix location
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LocationEnum {
    Far,
    Close,
    Middle
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Team {
    pub team_key: String,
    pub nickname: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Match {
    pub match_key: String,
    pub event_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Video {
    pub match_key: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct TBAMatch {
    pub match_key: String,
    pub event_key: String,
    pub time: chrono::NaiveTime,
    pub red_1: String,
    pub red_2: String,
    pub red_3: String,
    pub blue_1: String,
    pub blue_2: String,
    pub blue_3: String,
    pub video: i64,
}

// Pitscouting
#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct TeamEvent {
    pub id: i64,
    pub team_key: String,
    pub event_key: String,
    pub width: i16,
    pub length: i16,
    pub is_short: bool,
    pub is_camera: bool,
    pub drivetrain: String,
    pub is_ground_intake: bool,
    pub is_chute_intake: bool,
    pub polish: i16,
    pub scout_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Image {
    pub id: i64,
    pub event_key: String,
    pub team_key: String,
    pub url: String,
    pub scout_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct EventState {
    pub event_key: String,
    pub next_match: Option<String>,
    pub last_match: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum DriveTrain {
    Swerve,
    Tank,
    Other,
}
