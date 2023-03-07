use crate::db::models;
use crate::services;

#[derive(utoipa::OpenApi)]
#[openapi(
    info(title = "GitHab", description = "A habit tracking web application"),
    paths(services::hello::get_hello),
    components(schemas(models::Hello))
)]
pub struct ApiDocs;
