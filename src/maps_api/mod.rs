use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::Coordinate;

const PLACES_URL: &str = "https://places.googleapis.com/v1/places:searchNearby";
const NEARBY_RADIUS: f32 = 10.0;
const MAX_RESULTS: i32 = 10;

/// Top-level response envelope from the Places API.
#[derive(Debug, Deserialize)]
struct NearbySearchResponse {
    places: Option<Vec<NearbyPlace>>,
}

/// Represents a single place returned by the Nearby Search API.
#[derive(Debug, Serialize, Deserialize)]
pub struct NearbyPlace {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: DisplayName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayName {
    pub text: String,
    #[serde(rename = "languageCode")]
    pub language_code: String,
}

pub struct MapsClient {
    http_client: Client,
    api_key: String,
}

impl Clone for MapsClient {
    fn clone(&self) -> Self {
        Self {
            http_client: self.http_client.clone(),
            api_key: self.api_key.clone(),
        }
    }
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
    ) -> Result<Vec<NearbyPlace>, reqwest::Error> {
        let body = json!({
            "rankPreference": "DISTANCE",
            "maxResultCount": MAX_RESULTS,
            "locationRestriction": {
                "circle": {
                    "center": {
                        "latitude":  location.latitude,
                        "longitude": location.longitude
                    },
                    "radius": NEARBY_RADIUS
                }
            }
        });

        let response = self
            .http_client
            .post(PLACES_URL)
            .header("X-Goog-Api-Key", &self.api_key)
            .header("X-Goog-FieldMask", "places.displayName,places.id")
            .json(&body)
            .send()
            .await?
            .json::<NearbySearchResponse>()
            .await?;

        Ok(response.places.unwrap_or_default())
    }
}
