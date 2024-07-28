use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

use super::usecases;
use serde_json::json;

pub async fn find_one_product_handler(Path(product_id): Path<i32>) -> impl IntoResponse {
    match usecases::find_one_product(product_id) {
        super::entities::Result::Ok(p) => (StatusCode::OK, Json(p).into_response()),
        super::entities::Result::Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": 400,
                "message" : e,
            }))
            .into_response(),
        ),
    }
}
