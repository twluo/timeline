use axum::{
    Json, Router,
    extract::{Query, State},
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use chrono::Utc;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod database;
mod maps;
mod models;
mod utils;

use database::{DatabaseClient, TimelineEntry, TimelineQueryInput};
use maps::MapsClient;
use models::LogEntryInput;
use utils::coord_utils::get_distance;

#[derive(Clone)]
struct AppState {
    database: Arc<DatabaseClient>,
    maps: Arc<MapsClient>,
    nearby_radius_meters: f64,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let nearby_radius_meters: f64 = std::env::var("NEARBY_RADIUS_METERS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(50.0);

    let state = AppState {
        database: Arc::new(DatabaseClient::new()),
        maps: Arc::new(MapsClient::new()),
        nearby_radius_meters,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/log", post(log_handler))
        .route("/api/timeline", get(timeline_handler))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn log_handler(
    State(state): State<AppState>,
    Json(payload): Json<LogEntryInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !payload.coordinate.is_valid() {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            "coordinate out of range: latitude must be within -90..90, longitude within -180..180"
                .to_string(),
        ));
    }

    let timestamp = payload.timestamp.unwrap_or_else(Utc::now);

    // 1. Check the DB for cached nearby places
    let mut places = state
        .database
        .get_nearby_places(&payload.coordinate, state.nearby_radius_meters)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 2. Nothing cached — call the Maps API and update the DB
    if places.is_empty() {
        match state
            .maps
            .get_nearby(&payload.coordinate, state.nearby_radius_meters)
            .await
        {
            Ok(api_places) => {
                state
                    .database
                    .insert_places(&api_places)
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
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
    state
        .database
        .log_timeline(&TimelineEntry {
            user_id: payload.user_id,
            device_id: payload.device_id,
            place: closest.clone(),
            coordinate: payload.coordinate,
            timestamp,
        })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(closest))
}

async fn timeline_handler(
    State(state): State<AppState>,
    Query(params): Query<TimelineQueryInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let entries = state
        .database
        .query_timeline(params)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(entries))
}
