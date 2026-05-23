# Timeline

Back in the day, Google Maps had a feature called Timeline that allowed you to see where you'd been throughout the day and whether you'd visited a place before.

This is an attempt to recreate that for personal use, and an excuse to learn Rust.

## Structure

| Directory | Description |
|---|---|
| [`backend/`](./backend) | Rust REST API — ingests GPS coordinates, resolves nearby places, and serves timeline data |
| [`frontend/`](./frontend) | Vue web client — displays a day's stops as an interactive map |

## Requirements

- [Rust](https://rustup.rs/) (stable, 1.85+)
- [Bun](https://bun.sh) (or Node.js ≥ 20.19)
- A [Google Maps API key](https://console.cloud.google.com/google/maps-apis/credentials) with the **Places API (New)** and **Maps JavaScript API** enabled

## Getting Started

```sh
git clone https://github.com/twluo/timeline
cd timeline
```

### Backend

```sh
cd backend
cp .env.example .env  # add your GOOGLE_MAPS_API_KEY
cargo run
```

See [`backend/README.md`](./backend/README.md) for full setup and API docs.

### Frontend

```sh
cd frontend
bun install
cp .env.example .env  # add your VITE_GOOGLE_MAPS_API_KEY and VITE_TIMELINES_ENDPOINT
bun run dev
```

See [`frontend/README.md`](./frontend/README.md) for full setup and usage.

## Roadmap

- [x] `POST /api/log` — ingest a GPS coordinate and resolve its location
- [x] `GET /api/timeline` — get a chronological timeline for a given day
- [x] Map view of locations for a given day
- [ ] `GET /visits?placeId=` — visit history and frequency for a specific place
- [ ] Android and iOS clients
- [ ] Multi-device support
- [ ] Authentication
- [ ] Rate limiting
