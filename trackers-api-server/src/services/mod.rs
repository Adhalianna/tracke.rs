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
            license: Some(openapi::License {
                identifier: Some("MIT".to_owned()),
                ..openapi::License::default()
            }),
            ..openapi::Info::default()
        },
        ..openapi::OpenApi::default()
    };

    let api_router = api_router.finish_api(&mut openapi_doc);

    openapi_doc.tags.extend([openapi::Tag {
        name: "Registration".to_owned(),
        description: Some(
            "In order to register the client must start the registration process by sending his desired account details to the `/api/users` endpoint. This will cause a registration request to be issued and returned to the client. The request must be confirmed with a registration confirmation code that should be sent to the the provided email address."
                .to_owned(),
        ),
        ..openapi::Tag::default()
    }, openapi::Tag {
        name: "Log-in".to_owned(),
        description: Some(
            "In order to log in and use the account the client must use an `Authorization` header with bearer token which can be obtained at `/api/session/token`. The authentication process strives to implement OAuth2 with JWT used as the access token. The supported flows can be derived from the accepted by the application values for the `grant_type` field of the form used to start a new session."
                .to_owned(),
        ),
        ..openapi::Tag::default()
    }, openapi::Tag {
        name: "Task Management".to_owned(),
        description: Some(
            "The task management endpoints define the core features of the tracke.rs app. They allow the user to fetch their tasks, create new trackers, mark things done, etc."
                .to_owned(),
        ),
        ..openapi::Tag::default()
    }]);

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
