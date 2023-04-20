use aide::openapi::Response;

use crate::prelude::*;

pub struct ServerError(anyhow::Error);

/// The `ServerError` [`IntoResponse`](axum::response::IntoResponse) implementation
/// will hide the error cause for builds made in release mode.
impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, {
            #[cfg(not(debug_assertions))]
            {
                format!("An internal server error has occured!")
            }
            #[cfg(debug_assertions)]
            {
                format!("An internal server error has occured: {}", self.0)
            }
        })
            .into_response()
    }
}

/// Defines the schema used in the OpenAPI specification for the error.
impl JsonSchema for ServerError {
    fn schema_name() -> String {
        "server error".to_owned()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<String>()
    }
}

impl AideOperationOutput for ServerError {
    type Inner = Self;

    fn inferred_responses(
        _ctx: &mut aide::gen::GenContext,
        _operation: &mut openapi::Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![(
            Some(500),
            openapi::Response {
                description: String::from("An error has occured on the server."),
                ..openapi::Response::default()
            },
        )]
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
