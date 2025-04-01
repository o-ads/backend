use crate::db::{Event, EventMetadata, PlacementData};
use crate::errors::AppError;
use crate::server::AppState;
use crate::server::extractors::{ConnectAddr, EventData, SiteIdHeader, convert_ip, parse_occurred};
use axum::{
    Json,
    extract::{ConnectInfo, State},
};
use axum_extra::{headers::UserAgent, typed_header::TypedHeader};
use serde::{Deserialize, Serialize};

pub async fn post_event(
    State(AppState { db }): State<AppState>,
    ConnectInfo(addr): ConnectAddr,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    TypedHeader(SiteIdHeader(site_id)): TypedHeader<SiteIdHeader>,
    Json(event_data): Json<EventData>,
) -> Result<Json<EventResponse>, AppError> {
    let source_ip = convert_ip(addr)?;
    let metadata = EventMetadata {
        user_agent: user_agent.as_str().into(),
    };
    let occurred = parse_occurred(&event_data.occurred)?;
    Event::new(
        event_data.event_type,
        occurred,
        source_ip,
        metadata,
        event_data.ad_id,
        site_id,
    )
    .insert(&db)
    .await
    .map_err(|e: sqlx::Error| {
        tracing::error!("error posting event: {e}");
        if matches!(e, sqlx::Error::Database(_)) {
            AppError::InvalidRequest
        } else {
            AppError::InternalError
        }
    })?;
    Ok(Json(EventResponse { ok: true }))
}
pub async fn get_placement(
    State(AppState { db }): State<AppState>,
    TypedHeader(SiteIdHeader(site_id)): TypedHeader<SiteIdHeader>,
) -> Result<Json<PlacementData>, AppError> {
    let res = PlacementData::random_for_site(&db, site_id).await;
    match res {
        Ok(data) => Ok(Json(data)),
        Err(sqlx::Error::RowNotFound) => Err(AppError::NoRowReturned {
            entity_name: String::from("placement data"),
        }),
        Err(e) => {
            tracing::error!("database error: {e}");
            Err(AppError::InternalError)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventResponse {
    ok: bool,
}
