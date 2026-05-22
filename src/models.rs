use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const NEARBY_RADIUS_METERS: f64 = 50.0;

/// Coordinates struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Place {
    pub id: String,
    pub name: String,
    pub coordinate: Coordinate,
    pub address: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub user_id: i64,
    pub coordinate: Coordinate,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct TimelineEntry {
    pub user_id: i64,
    pub place: Place,
    pub coordinate: Coordinate,
    pub timestamp: DateTime<Utc>,
}
