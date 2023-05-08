use crate::prelude::*;

pub mod registration;
pub mod session;
pub mod task;
pub mod tracker;
pub mod user;

/// Mounts all the endpoints and deals with the extraction of OpenAPI
/// specification along with serving the documentation based on the OAS.
pub fn app_services() -> axum::Router<crate::AppState> {
    // collect services
    let api_router = ApiRouter::new().nest(
        "/api",
        ApiRouter::new()
            .merge(task::router())
            .merge(tracker::router())
            .merge(user::router())
            .merge(registration::router())
            .merge(session::router()),
    );

    // prep the OAS
    aide::gen::all_error_responses(true);
    let mut openapi_doc = openapi::OpenApi {
        info: openapi::Info {
            title: "tracke.rs".to_owned(),
            summary: Some(
                "A hackable task management web application implemented as a RESTful API service"
                    .to_owned(),
            ),
            description: None,
            contact: Some(openapi::Contact {
                name: Some(String::from("Natalia Goc")),
                url: Some(String::from("https://github.com/Adhalianna/tracke.rs")),
                ..openapi::Contact::default()
            }),
            ..openapi::Info::default()
        },
        ..openapi::OpenApi::default()
    };

    let api_router = api_router.finish_api(&mut openapi_doc);

    // serve the docs and the OAS
    api_router
        .route(
            "/openapi.json",
            axum::routing::get(serve_oas).layer(axum::Extension(openapi_doc)),
        )
        .route(
            "/doc",
            aide::redoc::Redoc::new("/openapi.json").axum_route().into(),
        )
}

async fn serve_oas(
    axum::Extension(oas): axum::Extension<openapi::OpenApi>,
) -> Json<openapi::OpenApi> {
    Json(oas)
}
