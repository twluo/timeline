# Timeline

Back in the day, Google Maps had a feature called Timeline that allowed you to see where you'd been throughout the day and whether you'd visited a place before.

This is an attempt to recreate that for personal use, and an excuse to learn Rust.

Built with [Axum](https://github.com/tokio-rs/axum), SQLite, and the Google Maps Places API.

## Features

- Log GPS coordinates from any client device (Android/iOS)
- Resolve nearby establishments using the Google Maps Places API
- Query a chronological timeline of locations for any given day
- Track visit frequency per establishment over time

## Requirements

- [Rust](https://rustup.rs/) (stable, 1.85+)
- A [Google Maps API key](https://developers.google.com/maps/documentation/places/web-service/get-api-key) with the **Places API (New)** enabled

## Getting Started

### 1. Clone the repo

```sh
git clone https://github.com/twluo/timeline
cd timeline
```

### 2. Configure environment variables

```sh
cp .env.example .env
```

Then open `.env` and fill in your values:

| Variable | Description |
|---|---|
| `GOOGLE_MAPS_API_KEY` | Your Google Maps API key |

### 3. Run

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
| `coordinate` | object | yes | `{ "latitude": float, "longitude": float }` |
| `timestamp` | string | no | ISO 8601 UTC timestamp - defaults to current time if omitted |

**Example**

```sh
curl -X POST http://localhost:3000/api/log \
  -H "Content-Type: application/json" \
  -d '{
    "userId": 1,
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
| `502` | Google Maps API request failed |

## Roadmap

- [x] `POST /api/log` - ingest a GPS coordinate and resolve its location
- [ ] `GET /day?date=YYYY-MM-DD` - get a timeline for a given day
- [ ] `GET /visits?place_id=` - visit history and frequency for a specific place
- [ ] Map view of locations for a given day
- [ ] Android and iOS clients
- [ ] Multi-device support
- [ ] Authentication
- [ ] Rate limiting
