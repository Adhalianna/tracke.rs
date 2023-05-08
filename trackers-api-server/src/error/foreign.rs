//! Conversions from foreign type into errors defined in the package

use super::*;

impl From<diesel::result::Error> for ApiError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::InvalidCString(err) => ServerError { err: err.into() }.into(),
            diesel::result::Error::DatabaseError(kind, info) => match kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => ConflictError {
                    msg: "attempted uniqness constraint violation".into(),
                    links: None,
                }
                .into(),
                diesel::result::DatabaseErrorKind::ForeignKeyViolation => ConflictError {
                    msg: "invalid relation among resources".into(),
                    links: None,
                }
                .into(),
                _ => ServerError {
                    err: anyhow::anyhow!(info.message().to_owned()),
                }
                .into(),
            },
            diesel::result::Error::NotFound => NotFoundError {
                msg: "failed to find the requested resource".into(),
                links: None,
            }
            .into(),
            _ => ServerError {
                err: anyhow::anyhow!(value.to_string()),
            }
            .into(),
        }
    }
}

impl From<deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>> for ApiError {
    fn from(
        value: deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>,
    ) -> Self {
        ServerError {
            err: anyhow::anyhow!(value),
        }
        .into()
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        ServerError { err: value.into() }.into()
    }
}
