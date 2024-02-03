use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::Uuid, Pool, Postgres};

use reqwest::Client as ReqwestClient;
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct Db {
    pub pool: Pool<Postgres>,
}

impl Db {
    pub async fn new(db_url: String) -> Result<Self, sqlx::Error> {
        let pool: Pool<Postgres> = PgPoolOptions::new().connect(&db_url).await?;

        let migrator = sqlx::migrate!();
        migrator.run(&pool).await?;

        Ok(Db { pool })
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub ctx: ReqwestClient,
    pub queue: RoboQueue,
}

/// Scouts: Access codes
#[derive(Debug, Clone)]
pub struct RoboQueue {
    pub match_keys: Vec<String>,
    pub robots: Vec<String>,
    pub scouts: Vec<String>,
}

// We're assuming that all three red and all three blue is the order of the robots
impl RoboQueue {
    /// Takes in a list of robots,
    /// Assigns each robot to one of the queued scouts, puts all the remaining robots in the robots queue
    pub async fn new_match_auto_assign(
        &mut self,
        mut robots: Vec<String>,
        db: &Db,
    ) -> Result<(), (QueueError, String)> {
        for (i, robot) in robots.iter().enumerate() {
            if self.scouts.is_empty() {
                self.robots.append(&mut robots);
                break;
            }
            let color = if robots.len() <= 3 {
                String::from("red")
            } else {
                String::from("blue")
            };
            let endpoint: String = Self::get_user(&self, self.scouts[i].clone(), db).await?;
            // Push robot and team color to user
        }
        Ok(())
    }

    // Manual assign
    pub async fn new_match_manual_assign(
        &mut self,
        robots: Vec<String>,
        scouts: Vec<String>,
        db: &Db,
    ) -> Result<(), (QueueError, String)> {
        assert_eq!(robots.len(), scouts.len());
        for (i, robot) in robots.iter().enumerate() {
            let color = if robots.len() <= 3 {
                String::from("red")
            } else {
                String::from("blue")
            };
            let endpoint: String = Self::get_user(&self, self.scouts[i].clone(), db).await?;

            // Push robot and team color to user
        }
        Ok(())
    }

    async fn get_user(&self, id: String, db: &Db) -> Result<String, (QueueError, String)> {
        match sqlx::query_as::<_, User>("SELECT * FROM \"Users\" WHERE id = $1")
            .bind(id)
            .fetch_one(&db.pool)
            .await
        {
            Ok(user) => {
                info!("Sent push notification to endpoint: {:?}", user);
                match user.endpoint {
                    Some(endpoint) => Ok(endpoint),
                    None => {
                        return Err((
                            QueueError::EndpointNotSet,
                            String::from("Scout endpoint not set, user must auth"),
                        ))
                    }
                }
            }
            Err(err) => {
                error!("{}", err);
                return Err((QueueError::QueryFailed, err.to_string()));
            }
        }
    }

    pub async fn add_scout_auto_assign(&mut self, scout: String) {
        if !self.robots.is_empty() {
            let robot = self.robots.pop();
            // Push robot to scout
        }

        self.scouts.push(scout);
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
    pub scout_id: String,
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
    pub drivetrain: DriveTrain,
    pub is_ground_intake: bool,
    pub is_chute_intake: bool,
    pub polish: Polish,
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
    pub next_match: String,
    pub last_match: String,
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
