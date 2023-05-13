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

pub use crate::response::{CreatedResource, DeletedResource, ModifiedResource, Resource};
pub use crate::{
    error::{
        ApiError, BadRequestError, ConflictError, ForbiddenError, GoneError, NotFoundError,
        ServerError, UnathorizedError,
    },
    json::JsonExtract,
    services, AppState,
};
pub use aide::{
    axum::routing, axum::ApiRouter, axum::AxumOperationHandler, axum::IntoApiResponse,
    axum::RouterExt, openapi, operation::OperationHandler as AideOperationHandler,
    operation::OperationInput as AideOperationInput,
    operation::OperationOutput as AideOperationOutput,
};
pub use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
pub use diesel::{
    AsChangeset, BoolExpressionMethods, ExpressionMethods, Identifiable, Insertable,
    PgArrayExpressionMethods, PgTextExpressionMethods, QueryDsl, Queryable,
};
pub use diesel_async::{AsyncConnection, RunQueryDsl, SaveChangesDsl};
pub use qs::axum::QsQuery;
pub use schemars::JsonSchema;
pub use serde::{Deserialize, Serialize};
pub use serde_qs as qs;
pub use std::{
    fmt::{Debug, Display},
    net::SocketAddr,
};
pub use trackers_models as models;
pub use trackers_models::db::schema as db_schema;
pub use trackers_models::types::Email as EmailAddress;
pub use trackers_models::types::Uuid as Base62Uuid;
