use crate::db::{AdEvent, ad::AdId, site::SiteId};
use crate::errors::AppError;
use axum::extract::ConnectInfo;
use axum_extra::headers::{self, Header, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use sqlx::types::{
    chrono::{DateTime, NaiveDateTime},
    ipnetwork::IpNetwork,
    uuid::Uuid,
};
use std::net::SocketAddr;

pub type ConnectAddr = ConnectInfo<SocketAddr>;

pub fn convert_ip(in_addr: SocketAddr) -> Result<IpNetwork, AppError> {
    IpNetwork::new(in_addr.ip(), 0).map_err(|_| {
        tracing::error!("bad addr {in_addr}");
        AppError::InvalidRequest
    })
}

pub fn parse_occurred(raw: &str) -> Result<NaiveDateTime, AppError> {
    DateTime::parse_from_rfc3339(raw)
        .map(|dt| dt.naive_utc())
        .map_err(|_| {
            tracing::error!("invalid occurred {raw}");
            AppError::InvalidRequest
        })
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventData {
    pub event_type: AdEvent,
    pub occurred: String,
    pub ad_id: AdId,
}

static SITE_ID_HEADER_NAME: HeaderName = HeaderName::from_static("x-o-ads-site-id");

pub struct SiteIdHeader(pub SiteId);

impl Header for SiteIdHeader {
    fn name() -> &'static HeaderName {
        &SITE_ID_HEADER_NAME
    }
    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;
        let value = value.to_str().map_err(|_| headers::Error::invalid())?;
        let uuid = Uuid::parse_str(value).map_err(|_| headers::Error::invalid())?;
        if uuid.get_version_num() == 4 {
            Ok(SiteIdHeader(uuid.into()))
        } else {
            Err(headers::Error::invalid())
        }
    }
    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let mut s: Vec<u8> = Vec::with_capacity(36);
        let w = self.0.as_uuid().as_hyphenated().encode_lower(&mut s);
        let value = HeaderValue::from_str(w).expect("decoding invalid site id");
        values.extend(std::iter::once(value));
    }
}
