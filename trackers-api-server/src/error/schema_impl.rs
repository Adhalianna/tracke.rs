use super::*;
use aide::openapi::Response;

impl AideOperationOutput for ApiError {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        _operation: &mut openapi::Operation,
    ) -> Vec<(Option<u16>, Response)> {
        Vec::new()
    }
}

impl JsonSchema for ServerError {
    fn schema_name() -> String {
        "server error".to_owned()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};

        let mut schema = gen.subschema_for::<super::ApiError>().into_object();
        schema.object.as_mut().and_then(|obj| {
            obj.properties
                .iter_mut()
                .find(|prop| prop.0 == "status")
                .and_then(|prop| {
                    let mut status_code_const_schema = SchemaObject::default();
                    status_code_const_schema.const_value =
                        Some(schemars::_serde_json::value::Number::from(500).into());
                    status_code_const_schema.instance_type =
                        Some(SingleOrVec::Single(Box::new(InstanceType::Number)));
                    *(prop.1) = Schema::Object(status_code_const_schema);
                    Some(prop)
                })
        });
        schemars::schema::Schema::Object(schema)
    }
}

impl AideOperationOutput for ServerError {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        _operation: &mut openapi::Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![(
            Some(500),
            openapi::Response {
                description: String::from("An error has occured on the server."),
                content: indexmap::indexmap! {
                "application/json".to_owned() => aide::openapi::MediaType{
                    schema: Some(aide::openapi::SchemaObject{json_schema: ctx.schema.subschema_for::<Self>(), external_docs: None, example: None}),
                    ..aide::openapi::MediaType::default()}
                },
                ..openapi::Response::default()
            },
        )]
    }
}

impl JsonSchema for BadRequestError {
    fn schema_name() -> String {
        "bad request error".to_owned()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};

        let mut schema = gen.subschema_for::<super::ApiError>().into_object();
        schema.object.as_mut().and_then(|obj| {
            obj.properties
                .iter_mut()
                .find(|prop| prop.0 == "status")
                .and_then(|prop| {
                    let mut status_code_const_schema = SchemaObject::default();
                    status_code_const_schema.const_value =
                        Some(schemars::_serde_json::value::Number::from(400).into());
                    status_code_const_schema.instance_type =
                        Some(SingleOrVec::Single(Box::new(InstanceType::Number)));
                    *(prop.1) = Schema::Object(status_code_const_schema);
                    Some(prop)
                })
        });
        schemars::schema::Schema::Object(schema)
    }
}

impl AideOperationOutput for BadRequestError {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        _operation: &mut openapi::Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![(
            Some(400),
            openapi::Response {
                description: String::from("Received a bad request."),
                content: indexmap::indexmap! {
                "application/json".to_owned() => aide::openapi::MediaType{
                    schema: Some(aide::openapi::SchemaObject{json_schema: ctx.schema.subschema_for::<Self>(), external_docs: None, example: None}),
                    ..aide::openapi::MediaType::default()}
                },
                ..openapi::Response::default()
            },
        )]
    }
}

impl JsonSchema for UnathorizedError {
    fn schema_name() -> String {
        "unauthorized error".to_owned()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};

        let mut schema = gen.subschema_for::<super::ApiError>().into_object();
        schema.object.as_mut().and_then(|obj| {
            obj.properties
                .iter_mut()
                .find(|prop| prop.0 == "status")
                .and_then(|prop| {
                    let mut status_code_const_schema = SchemaObject::default();
                    status_code_const_schema.const_value =
                        Some(schemars::_serde_json::value::Number::from(401).into());
                    status_code_const_schema.instance_type =
                        Some(SingleOrVec::Single(Box::new(InstanceType::Number)));
                    *(prop.1) = Schema::Object(status_code_const_schema);
                    Some(prop)
                })
        });
        schemars::schema::Schema::Object(schema)
    }
}

impl AideOperationOutput for UnathorizedError {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        _operation: &mut openapi::Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![(
            Some(401),
            openapi::Response {
                description: String::from("Attempted an unauthorized access."),
                content: indexmap::indexmap! {
                "application/json".to_owned() => aide::openapi::MediaType{
                    schema: Some(aide::openapi::SchemaObject{json_schema: ctx.schema.subschema_for::<Self>(), external_docs: None, example: None}),
                    ..aide::openapi::MediaType::default()}
                },
                ..openapi::Response::default()
            },
        )]
    }
}

impl JsonSchema for NotFoundError {
    fn schema_name() -> String {
        "not found error".to_owned()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};

        let mut schema = gen.subschema_for::<super::ApiError>().into_object();
        schema.object.as_mut().and_then(|obj| {
            obj.properties
                .iter_mut()
                .find(|prop| prop.0 == "status")
                .and_then(|prop| {
                    let mut status_code_const_schema = SchemaObject::default();
                    status_code_const_schema.const_value =
                        Some(schemars::_serde_json::value::Number::from(404).into());
                    status_code_const_schema.instance_type =
                        Some(SingleOrVec::Single(Box::new(InstanceType::Number)));
                    *(prop.1) = Schema::Object(status_code_const_schema);
                    Some(prop)
                })
        });
        schemars::schema::Schema::Object(schema)
    }
}

impl AideOperationOutput for NotFoundError {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        _operation: &mut openapi::Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![(
            Some(404),
            openapi::Response {
                description: String::from("Failed to find requested resource."),
                content: indexmap::indexmap! {
                "application/json".to_owned() => aide::openapi::MediaType{
                    schema: Some(aide::openapi::SchemaObject{json_schema: ctx.schema.subschema_for::<Self>(), external_docs: None, example: None}),
                    ..aide::openapi::MediaType::default()}
                },
                ..openapi::Response::default()
            },
        )]
    }
}

impl JsonSchema for ConflictError {
    fn schema_name() -> String {
        "conflict error".to_owned()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};

        let mut schema = gen.subschema_for::<super::ApiError>().into_object();
        schema.object.as_mut().and_then(|obj| {
            obj.properties
                .iter_mut()
                .find(|prop| prop.0 == "status")
                .and_then(|prop| {
                    let mut status_code_const_schema = SchemaObject::default();
                    status_code_const_schema.const_value =
                        Some(schemars::_serde_json::value::Number::from(409).into());
                    status_code_const_schema.instance_type =
                        Some(SingleOrVec::Single(Box::new(InstanceType::Number)));
                    *(prop.1) = Schema::Object(status_code_const_schema);
                    Some(prop)
                })
        });
        schemars::schema::Schema::Object(schema)
    }
}

impl AideOperationOutput for ConflictError {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        _operation: &mut openapi::Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![(
            Some(409),
            openapi::Response {
                description: String::from("Request in conflict with current state of a resource."),
                content: indexmap::indexmap! {
                "application/json".to_owned() => aide::openapi::MediaType{
                    schema: Some(aide::openapi::SchemaObject{json_schema: ctx.schema.subschema_for::<Self>(), external_docs: None, example: None}),
                    ..aide::openapi::MediaType::default()}
                },
                ..openapi::Response::default()
            },
        )]
    }
}
