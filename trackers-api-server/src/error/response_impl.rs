use super::*;
use crate::prelude::*;

impl IntoResponse for BadRequestError {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

impl IntoResponse for UnathorizedError {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::UNAUTHORIZED, Json(self)).into_response()
    }
}

impl IntoResponse for NotFoundError {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::NOT_FOUND, Json(self)).into_response()
    }
}

impl IntoResponse for ConflictError {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::CONFLICT, Json(self)).into_response()
    }
}

impl IntoResponse for ForbiddenError {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::FORBIDDEN, Json(self)).into_response()
    }
}
