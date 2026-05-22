use serde::{Deserialize, Serialize};

/// Coordinates struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    pub id: String,
    pub name: String,
    pub coordinate: Coordinate,
    pub address: String,
}
