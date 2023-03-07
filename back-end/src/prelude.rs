//! All commonly used imports.
//!
//! The `prelude` module contains many of commonly used
//! throughout the project imports. Using prelude is recommended
//! and can be achieved with the following line:
//! ```
//! use crate::prelude::*;
//! ```
//!
//! Many of required by some APIs traits are included in the prelude
//! so remember to try adding prelude import when a trait error
//! occurs.

pub use crate::{
    db::{models, schema},
    error::ServerError,
    services, AppState,
};
pub use axum::{extract::State, http::StatusCode, response::IntoResponse, routing, Json, Router};
pub use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
pub use diesel_async::{RunQueryDsl, SaveChangesDsl};
pub use serde::{Deserialize, Serialize};
pub use std::{
    fmt::{Debug, Display},
    net::SocketAddr,
};
pub use utoipa::ToSchema;
