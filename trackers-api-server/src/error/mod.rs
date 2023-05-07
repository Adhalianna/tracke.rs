use crate::prelude::*;
use std::collections::HashMap;

#[path = "user_err.rs"]
pub mod user;
pub use user::*;

#[path = "server_err.rs"]
pub mod server;
pub use server::*;

mod conv;
mod foreign;
mod schema_impl;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct ApiError {
    /// HTTP status code repeated once more
    pub status: u16,
    /// Error message
    pub msg: String,
    /// Links applicable in given situation
    pub links: Option<HashMap<&'static str, String>>,
}

pub trait ApiErrorTrait {
    fn status(&self) -> u16;
    fn msg(&self) -> String;
    fn links(&self) -> &Option<HashMap<&'static str, String>>;
}

/// An enum that can be used to construct the ApiError
pub enum Err {
    /// 40X family of errors - the client side errors.
    UserError(UserError),
    /// Simple 500 status error with no unneccessary details.
    ServerError(ServerError),
}

impl ApiErrorTrait for Err {
    fn status(&self) -> u16 {
        match self {
            Err::UserError(err) => err.status(),
            Err::ServerError(err) => err.status(),
        }
    }

    fn msg(&self) -> String {
        match self {
            Err::UserError(err) => err.msg(),
            Err::ServerError(err) => err.msg(),
        }
    }

    fn links(&self) -> &Option<HashMap<&'static str, String>> {
        match self {
            Err::UserError(err) => err.links(),
            Err::ServerError(err) => err.links(),
        }
    }
}
