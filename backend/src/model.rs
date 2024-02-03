use axum::extract::FromRef;
use cookie::Key;
use serde::Deserialize;
use sqlx::{
    postgres::{types::Oid, PgPoolOptions},
    Pool, Postgres,
};

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
    team: Team,
}

#[derive(Debug, Deserialize, Clone)]
struct Team {
    id: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub code: String,
}
