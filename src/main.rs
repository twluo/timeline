use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use rusqlite::Result;
use std::sync::Arc;

mod database;
mod maps;
mod models;
mod utils;

use database::DatabaseClient;
use maps::MapsClient;
use models::Coordinate;

#[derive(Clone)]
struct AppState {
    database: Arc<DatabaseClient>,
    maps: Arc<MapsClient>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let state = AppState {
        database: Arc::new(DatabaseClient::new()),
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

    state.database.insert_places(&places);
    Ok(Json(places))
}
