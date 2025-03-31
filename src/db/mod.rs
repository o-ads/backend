pub mod ad;
pub mod buyer;
pub mod event;
pub mod site;
use sqlx::postgres::PgPool;

pub type Db = std::sync::Arc<PgPool>;
pub type DbResult<T> = Result<T, sqlx::Error>;

pub use ad::{Ad, PlacementData};
pub use buyer::Buyer;
pub use event::{AdEvent, Event, EventMetadata};
pub use site::Site;

pub async fn pool(conn_str: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(conn_str).await
}

fn now() -> sqlx::types::chrono::NaiveDateTime {
    sqlx::types::chrono::Utc::now().naive_utc()
}
