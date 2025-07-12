use uuid::Uuid;
use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    Json,
};
use serde_json::Value;

use crate::types::{Db, CreateBooking, Booking, BookingFilters};

// POST /bookings
pub async fn create_booking(
    State(db): State<Db>,
    Json(payload): Json<CreateBooking>,
) -> Json<Booking> {
    let booking = Booking {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email,
        service_type: payload.service_type,
        date: payload.date,
        time_slot: payload.time_slot,
        status: "booked".to_string(),
    };

    db.write().await.insert(booking.id, booking.clone());
    Json(booking)
}


pub async fn list_bookings(
    State(db): State<Db>,
    Query(filters): Query<BookingFilters>,
) -> Json<Vec<Booking>> {
    let store = db.read().await;

    let filtered = store.values().filter(|booking| {
        let mut matches = true;

        if let Some(ref date) = filters.date {
            matches &= booking.date == *date;
        }

        if let Some(ref service) = filters.service_type {
            matches &= booking.service_type == *service;
        }

        matches
    });

    Json(filtered.cloned().collect())
}

/// GET /bookings/:id
pub async fn get_booking(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
) -> Result<Json<Booking>, StatusCode> {
    let store = db.read().await;

    match store.get(&id) {
        Some(booking) => Ok(Json(booking.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// PUT /bookings/:id
pub async fn update_booking(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
    Json(update): Json<Value>,
) -> Result<Json<Booking>, StatusCode> {
    // write‑lock because we’ll mutate
    let mut store = db.write().await;

    // 404 if the booking isn’t found
    let booking = store
        .get_mut(&id)
        .ok_or(StatusCode::NOT_FOUND)?;

    // Example: only allow status update for now
    if let Some(status) = update
        .get("status")
        .and_then(|v| v.as_str())
    {
        booking.status = status.to_owned();
    }

    Ok(Json(booking.clone()))
}

/// DELETE /bookings/:id
pub async fn delete_booking(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut store = db.write().await;

    if store.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT) // 204 — success, no body
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}