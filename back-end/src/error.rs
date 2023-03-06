use crate::prelude::*;

pub struct ServerError(anyhow::Error);

/// The `ServerError` [`IntoResponse`](axum::response::IntoResponse) implementation
/// will hide the error cause for builds made in release mode.
impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            #[cfg(not(debug_assertions))]
            format!("An internal server error has occured!"),
            #[cfg(debug_assertions)]
            format!("An internal server error has occured: {}", self.0),
        )
            .into_response()
    }
}

/// Makes conversion into ServerError possible with `?` for any type that
/// can also convert into [`anyhow::Error`].
impl<E> From<E> for ServerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
