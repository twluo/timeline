use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Coordinates struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn is_valid(&self) -> bool {
        self.latitude >= -90.0
            && self.latitude <= 90.0
            && self.longitude >= -180.0
            && self.longitude <= 180.0
    }
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
pub struct LogEntryInput {
    pub user_id: i64,
    pub device_id: String,
    pub coordinate: Coordinate,
    pub timestamp: Option<DateTime<Utc>>,
}
