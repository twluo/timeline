use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use chrono::Utc;
use std::sync::Arc;

mod database;
mod maps;
mod models;
mod utils;

use database::DatabaseClient;
use maps::MapsClient;
use models::{LogEntry, TimelineEntry};
use utils::coord_utils::get_distance;

#[derive(Clone)]
struct AppState {
    database: Arc<DatabaseClient>,
    maps: Arc<MapsClient>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let state = AppState {
        database: Arc::new(DatabaseClient::new()),
        maps: Arc::new(MapsClient::new()),
    };

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/log", post(log_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn log_handler(
    State(state): State<AppState>,
    Json(payload): Json<LogEntry>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let timestamp = payload.timestamp.unwrap_or_else(Utc::now);

    // 1. Check the DB for cached nearby places
    let mut places = state.database.get_nearby_places(&payload.coordinate);

    // 2. Nothing cached — call the Maps API and update the DB
    if places.is_empty() {
        match state.maps.get_nearby(&payload.coordinate).await {
            Ok(api_places) => {
                state.database.insert_places(&api_places);
                places = api_places;
            }
            Err(e) => return Err((StatusCode::BAD_GATEWAY, e.to_string())),
        }
    }

    // 3. Still nothing — no places exist near this coordinate
    if places.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            "No places found near this coordinate".to_string(),
        ));
    }

    // 4. Find the closest place
    let closest = places
        .iter()
        .min_by(|a, b| {
            get_distance(&payload.coordinate, &a.coordinate)
                .partial_cmp(&get_distance(&payload.coordinate, &b.coordinate))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap() // safe: is_empty check above guarantees at least one entry
        .clone();

    // 5. Log it to the timeline
    state.database.log_timeline(&TimelineEntry {
        user_id: payload.user_id,
        place: closest.clone(),
        coordinate: payload.coordinate,
        timestamp,
    });

    Ok(Json(closest))
}
