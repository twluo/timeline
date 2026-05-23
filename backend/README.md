# Timeline

The bread and butter of timeline. 

Built with [Axum](https://github.com/tokio-rs/axum), SQLite, and the Google Maps Places API.

## Features

- Log GPS coordinates from any client device
- Resolve nearby establishments using the Google Maps Places API
- Query a chronological timeline of locations for any given day
- Track visit frequency per establishment over time

## Requirements

- [Rust](https://rustup.rs/) (stable, 1.85+)
- A [Google Maps API key](https://developers.google.com/maps/documentation/places/web-service/get-api-key) with the **Places API (New)** enabled

## Getting Started

### 1. Configure environment variables

```sh
cp .env.example .env
```

Then open `.env` and fill in your values:

| Variable | Required | Default | Description |
|---|---|---|---|
| `GOOGLE_MAPS_API_KEY` | yes | — | Your Google Maps API key |
| `DATABASE_NAME` | no | `timeline.db` | SQLite database file path |
| `NEARBY_RADIUS_METERS` | no | `50` | Radius used when searching for nearby places |

### 2. Run

```sh
cargo run
```

The server starts on `http://127.0.0.1:3000`.

## API

### `GET /health`

Health check.

```
GET /health
→ 200 OK
```

---

### `POST /api/log`

Logs a timeline entry. Looks up nearby places from the local cache, falls back to the
Google Maps Places API if nothing is cached, and records the closest place to the timeline.
Returns the closest place found.

**Request body**

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | integer | yes | ID of the user logging the coordinate |
| `deviceId` | string | yes | Identifier of the device sending the coordinate |
| `coordinate` | object | yes | `{ "latitude": float, "longitude": float }` |
| `timestamp` | string | no | ISO 8601 UTC timestamp - defaults to current time if omitted |

**Example**

```sh
curl -X POST http://localhost:3000/api/log \
  -H "Content-Type: application/json" \
  -d '{
    "userId": 1,
    "deviceId": "device1",
    "coordinate": {
      "latitude": 37.42207167149138,
      "longitude": -122.08530966675468
    }
  }'
```

**Response**

```json
{
  "id": "ChIJj61dQgK6j4AR4GeTYWZsKWw",
  "name": "Googleplex",
  "address": "1600 Amphitheatre Pkwy, Mountain View, CA 94043, USA",
  "coordinate": {
    "latitude": 37.4220656,
    "longitude": -122.0840897
  }
}
```

**Errors**

| Status | Meaning |
|---|---|
| `400` | Missing or malformed request body |
| `404` | No places found near the coordinate |
| `422` | Coordinate values are out of range |
| `502` | Google Maps API request failed |

---

### `GET /api/timeline`

Returns a chronological list of timeline entries for a given user, device, and day.

**Query parameters**

| Parameter | Type | Required | Description |
|---|---|---|---|
| `userId` | integer | yes | ID of the user |
| `deviceId` | string | yes | Device identifier |
| `date` | string | no | Date in `YYYY-MM-DD` format - defaults to today (UTC) |

**Example**

```sh
curl "http://localhost:3000/api/timeline?userId=1&deviceId=device1&date=2025-05-22"
```

**Response**

```json
[
  {
    "user_id": 1,
    "device_id": "device1",
    "place": {
      "id": "ChIJj61dQgK6j4AR4GeTYWZsKWw",
      "name": "Googleplex",
      "address": "1600 Amphitheatre Pkwy, Mountain View, CA 94043, USA",
      "coordinate": {
        "latitude": 37.4220656,
        "longitude": -122.0840897
      }
    },
    "coordinate": {
      "latitude": 37.42207167149138,
      "longitude": -122.08530966675468
    },
    "timestamp": "2025-05-22T10:30:00Z"
  }
]
```

**Errors**

| Status | Meaning |
|---|---|
| `400` | Missing or malformed query parameters |
