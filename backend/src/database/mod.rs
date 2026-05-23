use crate::models::{Coordinate, Place};
use crate::utils::coord_utils::{get_bounding_box_from_source, get_distance};
use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};

#[derive(Debug, Serialize)]
pub struct TimelineEntry {
    pub user_id: i64,
    pub device_id: String,
    pub place: Place,
    pub coordinate: Coordinate,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineQueryInput {
    pub user_id: i64,
    pub device_id: String,
    // Date should be in the format of YYYY-MM-DD
    pub date: Option<NaiveDate>,
}

pub struct DatabaseClient {
    conn: Mutex<Connection>,
}

impl DatabaseClient {
    pub fn new() -> Self {
        let database_name = std::env::var("DATABASE_NAME").unwrap_or("timeline.db".to_string());
        let conn = Connection::open(database_name).expect("Error establishing connection");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS timeline (
              user_id   INTEGER,
              device_id TEXT,
              place_id  TEXT,
              latitude  REAL,
              longitude REAL,
              timestamp TEXT,
              PRIMARY KEY (user_id, device_id, place_id, timestamp)
            )",
            (),
        )
        .expect("failed to create timeline table");
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timeline_lookup
             ON timeline (user_id, device_id, timestamp)",
            (),
        )
        .expect("failed to create timeline index");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS places (
              id        TEXT PRIMARY KEY,
              latitude  REAL,
              longitude REAL,
              name      TEXT,
              address   TEXT
            )",
            (),
        )
        .expect("failed to create places table");
        Self {
            conn: Mutex::new(conn),
        }
    }

    fn lock(&self) -> MutexGuard<'_, Connection> {
        self.conn.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn log_timeline(&self, entry: &TimelineEntry) -> Result<(), rusqlite::Error> {
        let place = &entry.place;
        self.lock().execute(
            "INSERT OR IGNORE INTO timeline
             (user_id, device_id, place_id, latitude, longitude, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                entry.user_id,
                &entry.device_id,
                &place.id,
                entry.coordinate.latitude,
                entry.coordinate.longitude,
                entry.timestamp.to_rfc3339(),
            ),
        )?;
        Ok(())
    }

    pub fn query_timeline(
        &self,
        query: TimelineQueryInput,
    ) -> Result<Vec<TimelineEntry>, rusqlite::Error> {
        let date = query.date.unwrap_or_else(|| Utc::now().date_naive());
        let start = date.and_hms_opt(0, 0, 0).unwrap().and_utc().to_rfc3339();
        let end = (date + TimeDelta::days(1))
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .to_rfc3339();
        let conn = self.lock();

        let mut stmt = conn.prepare(
            "SELECT t.user_id, t.device_id, t.place_id,
                    t.latitude, t.longitude, t.timestamp,
                    p.name, p.address,
                    p.latitude, p.longitude
             FROM   timeline t
             JOIN   places p ON t.place_id = p.id
             WHERE  t.user_id   = ?1
             AND    t.device_id = ?2
             AND    t.timestamp >= ?3
             AND    t.timestamp <  ?4",
        )?;

        stmt.query_map((query.user_id, &query.device_id, &start, &end), |row| {
            let ts_str: String = row.get(5)?;
            let timestamp = DateTime::parse_from_rfc3339(&ts_str)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(|_| rusqlite::Error::InvalidQuery)?;
            Ok(TimelineEntry {
                user_id: row.get(0)?,
                device_id: row.get(1)?,
                coordinate: Coordinate {
                    latitude: row.get(3)?,
                    longitude: row.get(4)?,
                },
                place: Place {
                    id: row.get(2)?,
                    name: row.get(6)?,
                    address: row.get(7)?,
                    coordinate: Coordinate {
                        latitude: row.get(8)?,
                        longitude: row.get(9)?,
                    },
                },
                timestamp,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()
    }

    pub fn get_nearby_places(
        &self,
        coordinate: &Coordinate,
        radius_meters: f64,
    ) -> Result<Vec<Place>, rusqlite::Error> {
        let (north, south, east, west) = get_bounding_box_from_source(coordinate, radius_meters);

        let conn = self.lock();
        let mut stmt = conn.prepare(
            "SELECT id, name, latitude, longitude, address FROM places
             WHERE latitude  BETWEEN ?1 AND ?2
             AND   longitude BETWEEN ?3 AND ?4",
        )?;

        let all_places = stmt
            .query_map(
                (
                    south.latitude,
                    north.latitude,
                    west.longitude,
                    east.longitude,
                ),
                |row| {
                    Ok(Place {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        coordinate: Coordinate {
                            latitude: row.get(2)?,
                            longitude: row.get(3)?,
                        },
                        address: row.get(4)?,
                    })
                },
            )?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(all_places
            .into_iter()
            .filter(|p| get_distance(coordinate, &p.coordinate) <= radius_meters)
            .collect())
    }

    pub fn insert_places(&self, places: &[Place]) -> Result<(), rusqlite::Error> {
        let conn = self.lock();
        let mut stmt = conn.prepare(
            "INSERT OR IGNORE INTO places (id, latitude, longitude, name, address)
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
        for place in places {
            stmt.execute((
                &place.id,
                &place.coordinate.latitude,
                &place.coordinate.longitude,
                &place.name,
                &place.address,
            ))?;
        }
        Ok(())
    }
}
