use serde::{Deserialize, Serialize};

/// Coordinates struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}
