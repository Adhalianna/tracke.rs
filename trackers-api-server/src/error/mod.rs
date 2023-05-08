use crate::prelude::*;
use std::collections::HashMap;

#[path = "user_err.rs"]
pub mod user;
pub use user::*;

#[path = "server_err.rs"]
pub mod server;
pub use server::*;

mod foreign;
mod response_impl;
mod schema_impl;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct ApiError {
    /// HTTP status code repeated once more
    pub status: u16,
    /// Error message
    pub msg: String,
    /// Links applicable in given situation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<HashMap<&'static str, String>>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{status: {}, msg: {}}}", self.status, self.msg)
    }
}

impl std::error::Error for ApiError {}

pub trait ApiErrorTrait {
    fn status(&self) -> u16;
    fn msg(&self) -> String;
    fn links(&self) -> &Option<HashMap<&'static str, String>>;
}

impl<T> From<T> for ApiError
where
    T: ApiErrorTrait,
{
    fn from(value: T) -> Self {
        Self {
            status: value.status(),
            msg: value.msg(),
            links: value.links().to_owned(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::from_u16(self.status).unwrap(),
            Json(self),
        )
            .into_response()
    }
}
