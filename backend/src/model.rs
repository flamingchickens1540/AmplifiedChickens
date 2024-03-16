use axum::response::sse::Event;
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::watch::Sender;
use tokio::sync::Mutex;


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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AllianceColor {
    Red,
    Blue,
    None
}

#[derive(Debug, Clone)]
pub struct RoboQueue {
    pub match_keys: Vec<String>,
    // This is only for manual assignment
    pub assigned: HashMap<String, (String, AllianceColor)>, // access_token: team_key
    pub red_robots: Vec<String>,
    pub blue_robots: Vec<String>,
    pub curr_match_type: CurrentMatchType,
}

// We're assuming that all three red and all three blue is the order of the robots
impl RoboQueue {
    /// Takes in a list of robots,
    /// Assigns each robot to one of the queued scouts, puts all the remaining robots in the robots queue
    pub async fn new_match_auto_assign(
        &mut self,
        red_robots: Vec<String>,
        blue_robots: Vec<String>,
    ) {
        self.red_robots = red_robots;
        self.blue_robots = blue_robots;
        self.curr_match_type = CurrentMatchType::Auto;
    }

    pub fn scout_get_robot(&mut self, scout: String) -> Option<(String, AllianceColor)> {
        match self.assigned.get(&scout) {
            Some(team) => Some(team.clone()), // This might be removed if manual assignment is ditched
            None => match self.scout_get_red() {
                Some(robot) => Some((robot, AllianceColor::Red)),
                None => match self.scout_get_blue() {
                    Some(robot) => Some((robot, AllianceColor::Blue)),
                    None => None,
                },
            },
        }
    }

    pub fn scout_get_red(&mut self) -> Option<String> {
        self.red_robots.pop()
    }

    pub fn scout_get_blue(&mut self) -> Option<String> {
        self.blue_robots.pop()
    }
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
    pub scout_id: String,
    pub match_key: String,
    pub team_key: String,
    pub is_fielded: bool,
    pub is_leave_start: bool,
    pub auto_speaker_succeed: i16, //
    pub auto_speaker_missed: i16,  //
    pub auto_amp_succeed: i16,     //
    pub auto_amp_missed: i16,      //
    pub auto_piece_succeed: i16,   //
    pub auto_piece_missed: i16,    //
    pub tele_speaker_succeed: i16, //
    pub tele_speaker_missed: i16,  //
    pub tele_amp_succeed: i16,     //
    pub tele_amp_missed: i16,      //
    pub trap_succeed: i16,         // 
    pub trap_missed: i16,          //
    pub stage_enum: Stage,
    pub skill: i16,
    pub notes: String,
    pub is_broke: bool,
    pub is_died: bool,
}

#[derive(Serialize, Debug, Clone, Type, PartialEq, Deserialize, Default)]
#[sqlx(type_name = "stageenum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Stage {
    #[default]
    OnStage,
    #[sqlx(rename = "park")]
    Parked,
    #[sqlx(rename = "not attempted")]
    NotAttempted,
    Failed,
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
    pub drivetrain: DriveTrainEnum,
    pub is_ground_intake: bool,
    pub is_chute_intake: bool,
    pub notes: String,
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
#[serde(rename_all = "lowercase")]
pub enum DriveTrainEnum {
    Swerve,
    Tank,
    Other,
}
