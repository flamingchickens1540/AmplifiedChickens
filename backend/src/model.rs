use axum::{extract::FromRef, middleware, routing::get, Extension, Router};
use axum_login::{AuthUser, AuthnBackend, UserId};
use cookie::{Cookie, Key};
use serde::Deserialize;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    FromRow, Row,
};
use std::collections::HashMap;

use reqwest::Client as ReqwestClient;
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
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

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub pw_hash: Vec<u8>,
    pub coins: i64,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub code: String,
}
