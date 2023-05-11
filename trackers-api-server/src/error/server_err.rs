use super::*;

#[derive(Debug, Serialize)]
#[serde(from = "ApiError")]
pub struct ServerError {
    #[serde(skip)]
    pub err: anyhow::Error,
}

impl ApiErrorTrait for ServerError {
    fn status(&self) -> u16 {
        500
    }

    fn msg(&self) -> String {
        #[cfg(not(debug_assertions))]
        let msg = "An internal server error has occured!".to_owned();
        #[cfg(debug_assertions)]
        let msg = self.err.to_string();

        msg
    }

    fn links(&self) -> &Option<HashMap<&'static str, String>> {
        &None
    }
}

/// The `ServerError` [`IntoResponse`](axum::response::IntoResponse) implementation
/// will hide the error cause for builds made in release mode.
impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, {
            #[cfg(not(debug_assertions))]
            {
                Json(super::ApiError {
                    code: 500,
                    msg: format!("An internal server error has occured!"),
                    links: None,
                })
            }
            #[cfg(debug_assertions)]
            {
                Json(super::ApiError {
                    status: 500,
                    msg: format!("An internal server error has occured: {}", self.err),
                    links: None,
                })
            }
        })
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
        Self { err: err.into() }
    }
}
