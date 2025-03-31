use crate::db::{Db, DbResult, now};
use serde::{Deserialize, Serialize};
use sqlx::types::{chrono::NaiveDateTime, uuid::Uuid};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(transparent)]
pub struct SiteId(Uuid);

impl SiteId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SiteId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for SiteId {
    fn from(value: Uuid) -> SiteId {
        SiteId(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    id: SiteId,
    url: String,
    contact_email: String,
    created: NaiveDateTime,
    updated: NaiveDateTime,
}

impl std::cmp::PartialEq for Site {
    fn eq(&self, Site { id: rhs_id, .. }: &Site) -> bool {
        &self.id == rhs_id
    }
}

impl Site {
    pub fn new(url: String, contact_email: String) -> Self {
        let id = SiteId::new();
        let created: NaiveDateTime = now();
        let updated = created;
        Self {
            id,
            url,
            contact_email,
            created,
            updated,
        }
    }
    pub async fn insert(&self, db: &Db) -> DbResult<()> {
        let _ = sqlx::query!(
            "INSERT INTO site (id, url, contact_email, created, updated) VALUES ($1, $2, $3, $4, $5)",
            self.id.as_uuid(),
            self.url,
            self.contact_email,
            self.created,
            self.updated
        )
        .execute(db.as_ref())
        .await?;
        Ok(())
    }
}
