# Timeline

A simple client to communicate with the timeline backend. See the [backend API](../backend) for setup and usage.

Built with Vue3.

## Features

- Fetches timeline data from the timeline backend API
- Plots all stops as numbered, colour-coded pins (🟢 first → 🔵 middle → 🔴 last)
- Click any pin to see its name, address, and timestamp in a popup
- Prev / Next controls to step through stops in chronological order

## Prerequisites

- [Bun](https://bun.sh) (or Node.js ≥ 20.19)
- A [Google Maps API key](https://console.cloud.google.com/google/maps-apis/credentials) with the **Maps JavaScript API** enabled

## Setup

1. Install dependencies:

   ```sh
   bun install
   ```

2. Copy the example env file and fill in your values:

   ```sh
   cp .env.example .env
   ```

   | Variable | Description |
   |---|---|
   | `VITE_TIMELINES_ENDPOINT` | Base URL of the timeline API (e.g. `http://localhost:3000`) |
   | `VITE_GOOGLE_MAPS_API_KEY` | Google Maps JavaScript API key |

3. Start the dev server:

   ```sh
   bun run dev
   ```

## Usage

Open the app with the required query parameters:

```
http://localhost:5173/?userId=1&deviceId=device1&date=2026-05-23
```

| Parameter | Description |
|---|---|
| `userId` | The user whose timeline to display |
| `deviceId` | The device to query |
| `date` | Date in `YYYY-MM-DD` format |
