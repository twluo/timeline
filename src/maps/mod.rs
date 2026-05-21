use reqwest::Client;
use serde_json::{json, Value};

use crate::models::{Coordinate, Place};
use crate::utils::coord_utils::{get_distance};

const PLACES_URL: &str = "https://places.googleapis.com/v1/places:searchNearby";
const NEARBY_RADIUS: f32 = 50.0;
const MAX_RESULTS: i32 = 10;

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
                    "radius": NEARBY_RADIUS
                }
            }
        });

        let text = self
            .http_client
            .post(PLACES_URL)
            .header("X-Goog-Api-Key", &self.api_key)
            .header("X-Goog-FieldMask", "places.id,places.displayName,places.location,places.formattedAddress")
            .json(&body)
            .send()
            .await?
            .text()
            .await?;

        let root: Value = serde_json::from_str(&text).unwrap_or(Value::Null);

        let places: Vec<Place> = root["places"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|p| {
                Some(Place {
                    id: p["id"].as_str()?.to_string(),
                    name: p["displayName"]["text"].as_str()?.to_string(),
                    coordinate: Coordinate {
                        latitude: p["location"]["latitude"].as_f64()?,
                        longitude: p["location"]["longitude"].as_f64()?,
                    },
                    address: p["formattedAddress"].as_str()?.to_string(),
                })
            })
            .collect();
        let _ = get_distance(&places[0].coordinate, &places[1].coordinate);
        Ok(places)
    }
}
