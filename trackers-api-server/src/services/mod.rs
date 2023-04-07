use crate::prelude::*;

pub mod hello;

/// Nests all the api endpoints under __`/api`__ prefix and deals with the extraction
/// of OpenAPI specification along with serving the documentation based on the OAS.
pub fn app_services() -> axum::Router<crate::AppState> {
    // collect services
    let api_router = ApiRouter::new().nest("/api", ApiRouter::new().merge(hello::hello()));

    // prep the OAS
    let mut openapi_doc = openapi::OpenApi {
        info: openapi::Info {
            title: "tracke.rs".to_owned(),
            summary: Some("A hackable task management web application".to_owned()),
            description: None,
            ..openapi::Info::default()
        },
        ..openapi::OpenApi::default()
    };
    let api_router = api_router.finish_api(&mut openapi_doc);

    // serve the docs and the OAS
    api_router
        .route(
            "/api/openapi.json",
            axum::routing::get(serve_oas).layer(axum::Extension(openapi_doc)),
        )
        .route(
            "/api/doc",
            aide::redoc::Redoc::new("/api/openapi.json")
                .axum_route()
                .into(),
        )
}

async fn serve_oas(
    axum::Extension(oas): axum::Extension<openapi::OpenApi>,
) -> Json<openapi::OpenApi> {
    Json(oas)
}
