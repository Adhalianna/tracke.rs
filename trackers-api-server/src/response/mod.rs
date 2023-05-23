use std::collections::HashMap;

#[derive(serde::Serialize, schemars::JsonSchema)]
pub struct Resource<T> {
    pub data: T,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub links: HashMap<&'static str, String>,
}

impl<T> Resource<T> {
    pub fn insert_link(&mut self, name: &'static str, path: String) -> Option<String> {
        self.links.insert(name, path)
    }
    pub fn with_links(mut self, links: impl IntoIterator<Item = (&'static str, String)>) -> Self {
        self.links.extend(links.into_iter());
        self
    }
    pub fn new(data: T) -> Self {
        Self {
            data,
            links: HashMap::new(),
        }
    }
}

impl<T> From<T> for Resource<T> {
    fn from(value: T) -> Self {
        Self {
            data: value,
            links: HashMap::new(),
        }
    }
}

impl<T: serde::Serialize> axum::response::IntoResponse for Resource<T> {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}

impl<T: serde::Serialize + schemars::JsonSchema> aide::OperationOutput for Resource<T> {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        let mut resp = aide::openapi::Response::default();
        resp.content = indexmap::indexmap! {
            "application/json".to_owned() => aide::openapi::MediaType{
                schema: Some(aide::openapi::SchemaObject{
                    json_schema: ctx.schema.subschema_for::<Self>(),
                    external_docs: None,
                    example: None,
                }),
                ..aide::openapi::MediaType::default()
            }
        };
        resp.description = String::from("The resource has been successfully fetched.");
        vec![(Some(200), resp)]
    }
}

pub struct CreatedResource<T> {
    pub location: String,
    pub resource: Resource<T>,
}

impl<T: serde::Serialize> axum::response::IntoResponse for CreatedResource<T> {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::CREATED,
            [(axum::http::header::LOCATION, self.location)],
            axum::Json(self.resource),
        )
            .into_response()
    }
}

impl<T: serde::Serialize + schemars::JsonSchema> aide::OperationOutput for CreatedResource<T> {
    type Inner = Resource<T>;

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        let mut resp = aide::openapi::Response::default();
        resp.content = indexmap::indexmap! {
            "application/json".to_owned() => aide::openapi::MediaType{
                schema: Some(aide::openapi::SchemaObject{
                    json_schema: ctx.schema.subschema_for::<Resource<T>>(),
                    external_docs: None,
                    example: None,
                }),
                ..aide::openapi::MediaType::default()
            }
        };
        resp.description = String::from("The resource has been successfully created.");
        vec![(Some(201), resp)]
    }
}

pub struct ModifiedResource<T> {
    pub location: Option<String>,
    pub resource: Resource<T>,
}

impl<T: serde::Serialize> axum::response::IntoResponse for ModifiedResource<T> {
    fn into_response(self) -> axum::response::Response {
        let mut headers = axum::http::HeaderMap::new();
        if let Some(loc) = self.location {
            headers.insert(axum::http::header::LOCATION, loc.parse().unwrap());
        }
        (
            axum::http::StatusCode::CREATED,
            headers,
            axum::Json(self.resource),
        )
            .into_response()
    }
}

impl<T: serde::Serialize + schemars::JsonSchema> aide::OperationOutput for ModifiedResource<T> {
    type Inner = Resource<T>;

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        let mut resp = aide::openapi::Response::default();
        resp.content = indexmap::indexmap! {
            "application/json".to_owned() => aide::openapi::MediaType{
                schema: Some(aide::openapi::SchemaObject{
                    json_schema: ctx.schema.subschema_for::<Resource<T>>(),
                    external_docs: None,
                    example: None,
                }),
                ..aide::openapi::MediaType::default()
            }
        };
        resp.description = String::from("The resource has been successfully modified.");
        vec![(Some(200), resp)]
    }
}

#[derive(schemars::JsonSchema, Default, serde::Serialize)]
pub struct DeletedResource {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub links: HashMap<&'static str, String>,
}

impl axum::response::IntoResponse for DeletedResource {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}

impl aide::OperationOutput for DeletedResource {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        let mut resp = aide::openapi::Response::default();
        resp.content = indexmap::indexmap! {
            "application/json".to_owned() => aide::openapi::MediaType{
                schema: Some(aide::openapi::SchemaObject{
                    json_schema: ctx.schema.subschema_for::<Self>(),
                    external_docs: None,
                    example: None,
                }),
                ..aide::openapi::MediaType::default()
            }
        };
        resp.description = String::from("The resource has been successfully deleted.");
        vec![(Some(200), resp)]
    }
}
