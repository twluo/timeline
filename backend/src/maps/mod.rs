use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::models::{Coordinate, Place};

const PLACES_URL: &str = "https://places.googleapis.com/v1/places:searchNearby";
const MAX_RESULTS: i32 = 10;

#[derive(Deserialize)]
struct NearbySearchResponse {
    #[serde(default)]
    places: Vec<NearbyPlace>,
}

#[derive(Deserialize)]
struct NearbyPlace {
    id: String,
    #[serde(rename = "displayName")]
    display_name: DisplayName,
    location: ApiLocation,
    #[serde(rename = "formattedAddress", default)]
    formatted_address: String,
}

#[derive(Deserialize)]
struct DisplayName {
    text: String,
}

#[derive(Deserialize)]
struct ApiLocation {
    latitude: f64,
    longitude: f64,
}

#[derive(Clone)]
pub struct MapsClient {
    http_client: Client,
    api_key: String,
}

impl MapsClient {
    pub fn new() -> Self {
        let api_key =
            std::env::var("GOOGLE_MAPS_API_KEY").expect("GOOGLE_MAPS_API_KEY must be set in .env");
        let http_client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("reqwest client init");
        Self {
            http_client,
            api_key,
        }
    }

    pub async fn get_nearby(
        &self,
        location: &Coordinate,
        radius_meters: f64,
    ) -> Result<Vec<Place>, reqwest::Error> {
        let body = json!({
            "rankPreference": "DISTANCE",
            "maxResultCount": MAX_RESULTS,
            "locationRestriction": {
                "circle": {
                    "center": {
                        "latitude":  location.latitude,
                        "longitude": location.longitude
                    },
                    "radius": radius_meters
                }
            }
        });

        let response: NearbySearchResponse = self
            .http_client
            .post(PLACES_URL)
            .header("X-Goog-Api-Key", &self.api_key)
            .header(
                "X-Goog-FieldMask",
                "places.id,places.displayName,places.location,places.formattedAddress",
            )
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response
            .places
            .into_iter()
            .map(|p| Place {
                id: p.id,
                name: p.display_name.text,
                coordinate: Coordinate {
                    latitude: p.location.latitude,
                    longitude: p.location.longitude,
                },
                address: p.formatted_address,
            })
            .collect())
    }
}
