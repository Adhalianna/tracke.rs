use super::*;
use crate::prelude::*;
use std::collections::HashMap;

#[derive(Debug, JsonSchema)]
pub enum UserError {
    BadRequest(BadRequestError),
    Unauthorized(UnathorizedError),
    NotFound(NotFoundError),
    Conflict(ConflictError),
}

impl crate::error::ApiErrorTrait for UserError {
    fn status(&self) -> u16 {
        match self {
            UserError::BadRequest(e) => e.status(),
            UserError::Unauthorized(e) => e.status(),
            UserError::NotFound(e) => e.status(),
            UserError::Conflict(e) => e.status(),
        }
    }

    fn msg(&self) -> String {
        match self {
            UserError::BadRequest(e) => e.msg(),
            UserError::Unauthorized(e) => e.msg(),
            UserError::NotFound(e) => e.msg(),
            UserError::Conflict(e) => e.msg(),
        }
    }

    fn links(&self) -> &Option<HashMap<&'static str, String>> {
        match self {
            UserError::BadRequest(e) => e.links(),
            UserError::Unauthorized(e) => e.links(),
            UserError::NotFound(e) => e.links(),
            UserError::Conflict(e) => e.links(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(from = "ApiError")]
pub struct BadRequestError {
    pub msg: String,
    pub links: Option<HashMap<&'static str, String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(from = "ApiError")]
pub struct UnathorizedError {
    pub msg: String,
    pub links: Option<HashMap<&'static str, String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(from = "ApiError")]
pub struct NotFoundError {
    pub msg: String,
    pub links: Option<HashMap<&'static str, String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(from = "ApiError")]
pub struct ConflictError {
    pub msg: String,
    pub links: Option<HashMap<&'static str, String>>,
}

impl crate::error::ApiErrorTrait for BadRequestError {
    fn status(&self) -> u16 {
        400
    }
    fn msg(&self) -> String {
        self.msg.clone()
    }
    fn links(&self) -> &Option<HashMap<&'static str, String>> {
        &self.links
    }
}

impl crate::error::ApiErrorTrait for UnathorizedError {
    fn status(&self) -> u16 {
        401
    }
    fn msg(&self) -> String {
        self.msg.clone()
    }
    fn links(&self) -> &Option<HashMap<&'static str, String>> {
        &self.links
    }
}

impl crate::error::ApiErrorTrait for NotFoundError {
    fn status(&self) -> u16 {
        404
    }
    fn msg(&self) -> String {
        self.msg.clone()
    }
    fn links(&self) -> &Option<HashMap<&'static str, String>> {
        &self.links
    }
}

impl crate::error::ApiErrorTrait for ConflictError {
    fn status(&self) -> u16 {
        409
    }
    fn msg(&self) -> String {
        self.msg.clone()
    }
    fn links(&self) -> &Option<HashMap<&'static str, String>> {
        &self.links
    }
}
