use crate::db::{Db, DbResult, buyer::BuyerId, now, site::SiteId};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(transparent)]
pub struct AdId(Uuid);

impl AdId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for AdId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for AdId {
    fn from(value: Uuid) -> AdId {
        AdId(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ad {
    id: AdId,
    active: bool,
    asset_url_sm: String,
    asset_url_lg: String,
    clickthrough_url: String,
    created: NaiveDateTime,
    updated: NaiveDateTime,
    buyer_id: BuyerId,
}

impl std::cmp::PartialEq for Ad {
    fn eq(&self, Ad { id: rhs_id, .. }: &Ad) -> bool {
        &self.id == rhs_id
    }
}

impl Ad {
    pub fn new(
        buyer_id: BuyerId,
        asset_url_sm: String,
        asset_url_lg: String,
        clickthrough_url: String,
    ) -> Self {
        let id = AdId::new();
        let created = now();
        let updated = created;
        Self {
            id,
            active: false,
            asset_url_sm,
            asset_url_lg,
            clickthrough_url,
            created,
            updated,
            buyer_id,
        }
    }
    pub async fn insert(&self, db: &Db) -> DbResult<()> {
        let _ = sqlx::query!(
            "INSERT INTO ad (id, active, asset_url_sm, asset_url_lg, clickthrough_url, created, updated, buyer_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            self.id.as_uuid(),
            self.active,
            self.asset_url_sm,
            self.asset_url_lg,
            self.clickthrough_url,
            self.created,
            self.updated,
            self.buyer_id.as_uuid()
        )
        .execute(db.as_ref())
        .await?;
        Ok(())
    }
    pub async fn activate(db: &Db, id: AdId) -> DbResult<Self> {
        let updated = now();
        sqlx::query_as!(
            Ad,
            "UPDATE ad SET active = true, updated = $1 WHERE id = $2 RETURNING id, active, asset_url_sm, asset_url_lg, clickthrough_url, created, updated, buyer_id",
            updated,
            id.as_uuid()
        ).fetch_one(db.as_ref()).await
    }
    pub async fn deactivate(db: &Db, id: AdId) -> DbResult<Self> {
        let updated = now();
        sqlx::query_as!(
            Ad,
            "UPDATE ad SET active = false, updated = $1 WHERE id = $2 RETURNING id, active, asset_url_sm, asset_url_lg, clickthrough_url, created, updated, buyer_id",
            updated,
            id.as_uuid()
        ).fetch_one(db.as_ref()).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlacementData {
    ad_id: AdId,
    asset_url_sm: String,
    asset_url_lg: String,
    clickthrough_url: String,
}

impl PlacementData {
    pub async fn random_for_site(db: &Db, _site_id: SiteId) -> DbResult<Self> {
        // site_id currently unused but might eventually be
        sqlx::query_as!(
            PlacementData,
            r#"SELECT id AS ad_id, asset_url_sm, asset_url_lg, clickthrough_url
               FROM ad
               TABLESAMPLE SYSTEM(100)
               WHERE active = true
               LIMIT 1"#
        )
        .fetch_one(db.as_ref())
        .await
    }
}
