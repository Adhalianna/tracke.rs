use utoipa::OpenApi;

pub mod hello;

/// Creates a router with all of the applications services (sub-routers) nested under
/// __`/api`__ prefix.
pub fn app_services() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .nest("/api", axum::Router::new().merge(hello::hello()))
        .merge(
            utoipa_swagger_ui::SwaggerUi::new("/api/doc")
                .url("/api/doc/openapi.json", crate::docs::ApiDocs::openapi()),
        )
}
