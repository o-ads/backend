use crate::db::{Db, DbResult, now};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(transparent)]
pub struct BuyerId(Uuid);

impl BuyerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for BuyerId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for BuyerId {
    fn from(value: Uuid) -> BuyerId {
        BuyerId(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Buyer {
    id: BuyerId,
    contact_email: String,
    created: NaiveDateTime,
    updated: NaiveDateTime,
}

impl std::cmp::PartialEq for Buyer {
    fn eq(&self, Buyer { id: rhs_id, .. }: &Buyer) -> bool {
        &self.id == rhs_id
    }
}

impl Buyer {
    pub fn new(contact_email: String) -> Self {
        let id = BuyerId::new();
        let created = now();
        let updated = created;
        Self {
            id,
            contact_email,
            created,
            updated,
        }
    }
    pub async fn insert(&self, db: &Db) -> DbResult<()> {
        let _ = sqlx::query!(
            "INSERT INTO buyer (id, contact_email, created, updated) VALUES ($1, $2, $3, $4)",
            self.id.as_uuid(),
            self.contact_email,
            self.created,
            self.updated
        )
        .execute(db.as_ref())
        .await?;
        Ok(())
    }
}
