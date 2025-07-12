
#[cfg(test)]
mod tests {
    use crate::app;
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use serde_json::json;
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn create_and_list_booking() {
        let app = app(); // new router each test

        // -------- POST /bookings -----------
        let payload = json!({
            "name": "Alice",
            "email": "alice@example.com",
            "service_type": "Plumbing",
            "date": "2025-07-20",
            "time_slot": "10:00-11:00"
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/bookings")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Grab response body
        let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let created: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        // basic field checks
        assert_eq!(created["name"], "Alice");
        assert_eq!(created["status"], "booked");

        // -------- GET /bookings -----------
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/bookings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let list: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(list.as_array().unwrap().len(), 1);
        assert_eq!(list[0]["email"], "alice@example.com");
    }

    #[tokio::test]
    async fn get_booking_found_and_not_found() {
        let app = app();

        // Step 1: Create a booking
        let payload = json!({
            "name": "Alice",
            "email": "alice@example.com",
            "service_type": "Massage",
            "date": "2025-07-21",
            "time_slot": "13:00-14:00"
        });

        let res = app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/bookings")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        ).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body = to_bytes(res.into_body(), 1024 * 1024).await.unwrap();
        let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = created["id"].as_str().unwrap();

        // Step 2: Get the booking (should succeed)
        let res = app.clone().oneshot(
            Request::builder()
                .method("GET")
                .uri(&format!("/bookings/{id}"))
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        // Step 3: Try an invalid ID (should 404)
        let res = app.clone().oneshot(
            Request::builder()
                .method("GET")
                .uri("/bookings/00000000-0000-0000-0000-000000000000")
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn update_booking_status() {
        let app = app();

        // Create a booking
        let payload = json!({
            "name": "Bob",
            "email": "bob@example.com",
            "service_type": "Haircut",
            "date": "2025-07-22",
            "time_slot": "15:00-16:00"
        });

        let res = app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/bookings")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        ).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body = to_bytes(res.into_body(), 1024 * 1024).await.unwrap();
        let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = created["id"].as_str().unwrap();

        // Update the booking's status to "cancelled"
        let update = json!({ "status": "cancelled" });

        let res = app.clone().oneshot(
            Request::builder()
                .method("PUT")
                .uri(&format!("/bookings/{id}"))
                .header("content-type", "application/json")
                .body(Body::from(update.to_string()))
                .unwrap(),
        ).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let updated_body = to_bytes(res.into_body(), 1024 * 1024).await.unwrap();
        let updated: serde_json::Value = serde_json::from_slice(&updated_body).unwrap();

        assert_eq!(updated["status"], "cancelled");

        // Try to update a non-existent booking
        let res = app.clone().oneshot(
            Request::builder()
                .method("PUT")
                .uri("/bookings/00000000-0000-0000-0000-000000000000")
                .header("content-type", "application/json")
                .body(Body::from(update.to_string()))
                .unwrap(),
        ).await.unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_booking_works() {
        let app = app();

        // Create a booking
        let payload = json!({
            "name": "Eve",
            "email": "eve@example.com",
            "service_type": "Yoga",
            "date": "2025-07-25",
            "time_slot": "18:00-19:00"
        });

        let res = app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/bookings")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        ).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body = to_bytes(res.into_body(), 1024 * 1024).await.unwrap();
        let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = created["id"].as_str().unwrap();

        // DELETE the booking
        let res = app.clone().oneshot(
            Request::builder()
                .method("DELETE")
                .uri(&format!("/bookings/{id}"))
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap();

        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        // Verify it’s gone
        let res = app.clone().oneshot(
            Request::builder()
                .method("GET")
                .uri(&format!("/bookings/{id}"))
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
