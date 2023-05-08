//! Custom JSON extractor
use crate::prelude::*;
use serde::de::DeserializeOwned;

/// The default JSON extractor for the API server which rejection can be customised by
/// the `R` type parameter.
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct JsonExtract<T: DeserializeOwned, R: FromJsonRejection = ApiJsonRejection> {
    pub data: T,
    rej: std::marker::PhantomData<fn() -> R>,
}

impl<T: DeserializeOwned, R: FromJsonRejection> JsonExtract<T, R> {
    pub fn extract(self) -> T {
        self.data
    }
}

pub trait FromJsonRejection {
    type NewRejection: Serialize + axum::response::IntoResponse;
    fn from_rejection(rejection: axum::extract::rejection::JsonRejection) -> Self::NewRejection;
}

impl<T: DeserializeOwned + JsonSchema, R: FromJsonRejection + aide::OperationOutput>
    aide::OperationInput for JsonExtract<T, R>
{
    fn operation_input(ctx: &mut aide::gen::GenContext, operation: &mut openapi::Operation) {
        let inferred = Self::inferred_early_responses(ctx, operation)
            .into_iter()
            .map(|(status, resp)| {
                (
                    aide::openapi::StatusCode::Code(status.unwrap()),
                    aide::openapi::ReferenceOr::Item(resp),
                )
            });
        operation
            .responses
            .get_or_insert(aide::openapi::Responses::default())
            .responses
            .extend(inferred);
        axum::Json::<T>::operation_input(ctx, operation)
    }

    fn inferred_early_responses(
        ctx: &mut aide::gen::GenContext,
        operation: &mut openapi::Operation,
    ) -> Vec<(Option<u16>, openapi::Response)> {
        let mut responses = Vec::new();
        responses.extend(R::inferred_responses(ctx, operation));
        responses
    }
}

/// The default json rejection for the API server.
pub struct ApiJsonRejection;

impl FromJsonRejection for ApiJsonRejection {
    type NewRejection = BadRequestError;

    fn from_rejection(rejection: axum::extract::rejection::JsonRejection) -> Self::NewRejection {
        match rejection {
            axum::extract::rejection::JsonRejection::JsonDataError(err) => {
                BadRequestError::default()
                    .with_msg(
                        err.body_text()
                            .strip_prefix(&(err.to_string() + ": "))
                            .unwrap(),
                    )
                    .with_docs()
            }
            axum::extract::rejection::JsonRejection::JsonSyntaxError(err) => {
                BadRequestError::default()
                    .with_msg(err.body_text())
                    .with_docs()
            }
            axum::extract::rejection::JsonRejection::MissingJsonContentType(err) => {
                BadRequestError::default()
                    .with_msg(err.body_text())
                    .with_docs()
            }
            axum::extract::rejection::JsonRejection::BytesRejection(err) => {
                BadRequestError::default()
                    .with_msg(err.body_text())
                    .with_docs()
            }
            _ => BadRequestError::default()
                .with_msg("cannot process provided payload")
                .with_docs(),
        }
    }
}

#[axum::async_trait]
impl<T, R, B, S> axum::extract::FromRequest<S, B> for JsonExtract<T, R>
where
    T: DeserializeOwned,
    R: FromJsonRejection,
    B: Send + 'static + axum::body::HttpBody,
    B::Data: Send,
    B::Error: std::error::Error + Send + Sync,
    S: Send + Sync,
{
    type Rejection = R::NewRejection;
    async fn from_request(req: axum::http::Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(good_json) => Ok(Self {
                data: good_json.0,
                rej: std::marker::PhantomData::default(),
            }),
            Err(rej) => Err(R::from_rejection(rej)),
        }
    }
}

impl aide::OperationOutput for ApiJsonRejection {
    type Inner = BadRequestError;
    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        operation: &mut openapi::Operation,
    ) -> Vec<(Option<u16>, openapi::Response)> {
        let mut standard_bad_req_response = BadRequestError::operation_response(ctx, operation).expect("expected the BadRequestError type's implementation of aide::OpertaionOutput to return Some on operation_response");
        standard_bad_req_response.description =
            "Provided JSON body could not be processed or it does not meet the requirements given in the specification."
                .to_owned();
        vec![(Some(400), standard_bad_req_response)]
    }
}
