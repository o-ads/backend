use crate::db::{Db, DbResult, ad::AdId, site::SiteId};
use serde::{Deserialize, Serialize};
use sqlx::types::{Json, chrono::NaiveDateTime, ipnetwork::IpNetwork};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(rename_all = "lowercase")]
pub enum AdEvent {
    Load,
    View,
    Click,
}

impl AdEvent {
    fn as_str(&self) -> &str {
        match self {
            Self::Load => "load",
            Self::View => "view",
            Self::Click => "click",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(transparent)]
pub struct EventId(Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for EventId {
    fn from(value: Uuid) -> EventId {
        EventId(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventMetadata {
    pub user_agent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    id: EventId,
    event: AdEvent,
    occurred: NaiveDateTime,
    source_ip: IpNetwork,
    metadata: Json<EventMetadata>,
    ad_id: AdId,
    site_id: SiteId,
}

impl std::cmp::PartialEq for Event {
    fn eq(&self, Event { id: rhs_id, .. }: &Event) -> bool {
        &self.id == rhs_id
    }
}

impl Event {
    pub fn new(
        event: AdEvent,
        occurred: NaiveDateTime,
        source_ip: IpNetwork,
        metadata: EventMetadata,
        ad_id: AdId,
        site_id: SiteId,
    ) -> Self {
        let id = EventId::new();
        Self {
            id,
            event,
            occurred,
            source_ip,
            metadata: Json(metadata),
            ad_id,
            site_id,
        }
    }
    pub async fn insert(&self, db: &Db) -> DbResult<()> {
        let md_json = serde_json::json!(self.metadata);
        let _ = sqlx::query!("INSERT INTO event (id, event, occurred, source_ip, metadata, ad_id, site_id) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            self.id.as_uuid(),
            self.event.as_str(),
            self.occurred,
            self.source_ip,
            md_json,
            self.ad_id.as_uuid(),
            self.site_id.as_uuid()
        ).execute(db.as_ref()).await?;
        Ok(())
    }
}
