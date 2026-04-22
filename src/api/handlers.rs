use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::sync::Arc;
use crate::core::service::FaceService;

pub async fn enroll_handler(
    State(service): State<Arc<FaceService>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut user_id: Option<String> = None;
    let mut image_bytes: Option<Vec<u8>> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name == "user_id" {
            user_id = Some(field.text().await.unwrap_or_default());
        } else if name == "image" {
            image_bytes = Some(field.bytes().await.unwrap_or_default().to_vec());
        }
    }

    match (user_id, image_bytes) {
        (Some(uid), Some(img)) => {
            match service.enroll_user(&uid, &img).await {
                Ok(_) => (StatusCode::OK, Json(json!({"status": "success"}))),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
            }
        }
        _ => (StatusCode::BAD_REQUEST, Json(json!({"error": "missing fields"}))),
    }
}

pub async fn identify_handler(
    State(service): State<Arc<FaceService>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut image_bytes: Option<Vec<u8>> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name == "image" {
            image_bytes = Some(field.bytes().await.unwrap_or_default().to_vec());
        }
    }

    match image_bytes {
        Some(img) => {
            match service.identify_user(&img).await {
                Ok(Some(result)) => (StatusCode::OK, Json(json!({"status": "found", "user_id": result.user_id, "score": result.score}))),
                Ok(None) => (StatusCode::OK, Json(json!({"status": "not_found"}))),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
            }
        }
        _ => (StatusCode::BAD_REQUEST, Json(json!({"error": "missing image"}))),
    }
}
