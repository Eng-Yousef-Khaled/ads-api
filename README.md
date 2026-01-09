# ads-api

Simple Rust HTTP API for managing ads and tracking clicks.

## Overview
A small REST service to create, list and track clicks on advertisements. Implements controllers, models and routes in Rust.

## Features
- Create and list ads
- Record ad clicks
- Country lookup helper

## Tech stack
- Rust (Cargo)
- HTTP server framework (e.g. actix-web / axum — see code for exact crate)
- Database (Postgres/MySQL/SQLite — set via env)
- SQL x/ORM (see model/queries.rs)

## Quick start

Prerequisites
- Rust toolchain (rustc + cargo)
- A running database (Postgres/MySQL/SQLite) configured by env vars

Install and run
1. Clone repo
2. Configure environment variables (example)
    - DATABASE_URL=<your-db-url>
    - PORT=8080
    - RUST_LOG=info
3. Build and run
    - Development: `cargo run`
    - Production: `cargo run --release`
4. Server listens on `http://0.0.0.0:${PORT:-8080}`

(If the project uses migrations, run them before starting the server.)

## API (example)
- GET /ads — list ads
- POST /ads — create ad (JSON body)
- POST /click_ads — record a click (JSON body)
- GET /ads/:id — get ad by id

Check route implementations in `src/route/` for exact request/response shapes.

## Project layout
- src/
  - main.rs — application entry
  - controller.rs — top-level controller wiring
  - route.rs — route registration
  - model.rs — shared model types
  - error.rs — error types
  - controller/ — per-controller code
     - get_country.rs
  - model/
     - ads.rs
     - queries.rs
  - route/
     - ads.rs
     - click_ads.rs

## Development tips
- Inspect `src/route/*.rs` and `src/model/*.rs` for JSON schemas
- Use `RUST_LOG=debug` to get detailed logs
- Add DB migrations as needed and ensure `DATABASE_URL` is correct

## Tests
No tests included by default. Add `cargo test` suites as needed.

## Contributing
Open issues and PRs. Keep changes small and well-tested.

## License
Specify a license in the repository root (e.g. MIT, Apache-2.0).

For exact environment keys, request/response JSON and dependencies, consult the source files under `src/`.