use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

mod maps_api;
mod models;

use maps_api::MapsClient;
use models::Coordinate;

const DATABASE_URL: &str = "timeline.db";

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
    maps: Arc<MapsClient>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let conn = Connection::open(DATABASE_URL)?;

    let state = AppState {
        db: Arc::new(Mutex::new(conn)),
        maps: Arc::new(MapsClient::new()),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/nearby", get(nearby_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn nearby_handler(
    State(state): State<AppState>,
    Query(params): Query<Coordinate>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if params.latitude < -90.0 || params.latitude > 90.0 {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            "latitude must be between -90 and 90".to_string(),
        ));
    }
    if params.longitude < -180.0 || params.longitude > 180.0 {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            "longitude must be between -180 and 180".to_string(),
        ));
    }

    let places = state
        .maps
        .get_nearby(&params)
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    Ok(Json(places))
}
