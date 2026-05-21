use crate::models::Coordinate;
use crate::models::Place;
use crate::utils::coord_utils::{get_bounding_box_from_source, get_distance};
use rusqlite::Connection;
use std::sync::{Mutex, MutexGuard};

pub struct DatabaseClient {
    conn: Mutex<Connection>,
}

impl DatabaseClient {
    pub fn new() -> Self {
        let database_name = std::env::var("DATABASE_NAME").unwrap_or("timeline.db".to_string());
        let conn = Connection::open(database_name).expect("Error establishing connection");
        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS places (
              id        TEXT PRIMARY KEY,
              latitude  REAL,
              longitude REAL,
              name      TEXT,
              address   TEXT
            )",
            (),
        );
        Self {
            conn: Mutex::new(conn),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, Connection> {
        self.conn.lock().unwrap()
    }

    pub fn get_nearby_places(&self, coordinate: &Coordinate) -> Vec<Place> {
        const RADIUS_METERS: f64 = 10.0;

        let (north, south, east, west) = get_bounding_box_from_source(coordinate, RADIUS_METERS);

        let conn = self.lock();
        let mut stmt = conn
            .prepare(
                "SELECT id, name, latitude, longitude, address FROM places
                 WHERE latitude  BETWEEN ?1 AND ?2
                 AND   longitude BETWEEN ?3 AND ?4",
            )
            .expect("failed to prepare nearby places query");

        stmt.query_map(
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
        )
        .expect("nearby places query failed")
        .filter_map(|r| r.ok())
        .filter(|p| get_distance(coordinate, &p.coordinate) <= RADIUS_METERS)
        .collect()
    }

    pub fn insert_places(&self, places: &Vec<Place>) {
        let conn = self.lock();
        let mut stmt = conn
            .prepare(
                "INSERT OR IGNORE INTO places (id, latitude, longitude, name) VALUES (?1, ?2, ?3, ?4)",
            )
            .expect("failed to prepare insert statement");

        for place in places {
            let _ = stmt.execute((
                &place.id,
                &place.coordinate.latitude,
                &place.coordinate.longitude,
                &place.name,
            ));
        }
    }
}
