use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio::sync::{RwLock};
use std::collections::HashMap;
use std::sync::Arc;

pub type Db = Arc<RwLock<HashMap<Uuid, Booking>>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Booking {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub service_type: String,
    pub date: String,
    pub time_slot: String,
    pub status: String, // e.g., "confirmed", "cancelled", "pending"
}

#[derive(Deserialize)]
pub struct BookingFilters {
    pub date: Option<String>,
    pub service_type: Option<String>,
}


#[derive(Deserialize)]
pub struct CreateBooking {
    pub name: String,
    pub email: String,
    pub service_type: String,
    pub date: String,
    pub time_slot: String,
}
