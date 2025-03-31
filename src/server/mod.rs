pub(crate) mod extractors;
pub mod handlers;
use crate::db::{Db, pool};
use axum::{
    Router,
    routing::{get, post},
};
use handlers::{get_placement, post_event};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}
impl AppState {
    pub async fn new(conn_str: &str) -> Result<Self, sqlx::Error> {
        let p = pool(conn_str).await?;
        Ok(Self { db: Arc::new(p) })
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/event", post(post_event))
        .route("/placement", get(get_placement))
}
