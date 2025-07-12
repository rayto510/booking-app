use axum::{routing::{get, post}, Router};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;

pub mod types;
use types::{Booking};

pub mod routes;
use routes::{create_booking, list_bookings, get_booking, update_booking, delete_booking};

type Db = Arc<RwLock<HashMap<Uuid, Booking>>>;

#[cfg(test)]
mod tests;

/// Build a fresh router with its own in‑memory DB
pub fn app() -> Router {
    let db: Db = Arc::new(RwLock::new(HashMap::new()));

   let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS, // add this line
        ]);

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/bookings",
            post(create_booking)
            .get(list_bookings)
        )
        .route("/bookings/:id",
            get(get_booking)
            .put(update_booking)  // ← now compiles
            .delete(delete_booking)
        )
        .with_state(db)
        .layer(cors)
}
