use super::*;
use crate::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(into = "ApiError")]
pub struct BadRequestError {
    pub msg: String,
    pub links: Option<HashMap<&'static str, String>>,
}

impl BadRequestError {
    pub fn with_msg(mut self, msg: impl std::fmt::Display) -> Self {
        self.msg = msg.to_string();
        self
    }
    pub fn with_links(mut self, links: impl IntoIterator<Item = (&'static str, String)>) -> Self {
        self.links.get_or_insert(HashMap::default()).extend(links);
        self
    }
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(into = "ApiError")]
pub struct UnathorizedError {
    pub msg: String,
    pub links: Option<HashMap<&'static str, String>>,
}

impl UnathorizedError {
    pub fn with_msg(mut self, msg: impl std::fmt::Display) -> Self {
        self.msg = msg.to_string();
        self
    }
    pub fn with_links(mut self, links: impl IntoIterator<Item = (&'static str, String)>) -> Self {
        self.links.get_or_insert(HashMap::default()).extend(links);
        self
    }
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(into = "ApiError")]
pub struct NotFoundError {
    pub msg: String,
    pub links: Option<HashMap<&'static str, String>>,
}

impl NotFoundError {
    pub fn with_msg(mut self, msg: impl std::fmt::Display) -> Self {
        self.msg = msg.to_string();
        self
    }
    pub fn with_links(mut self, links: impl IntoIterator<Item = (&'static str, String)>) -> Self {
        self.links.get_or_insert(HashMap::default()).extend(links);
        self
    }
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(into = "ApiError")]
pub struct ConflictError {
    pub msg: String,
    pub links: Option<HashMap<&'static str, String>>,
}

impl ConflictError {
    pub fn with_msg(mut self, msg: impl std::fmt::Display) -> Self {
        self.msg = msg.to_string();
        self
    }
    pub fn with_links(mut self, links: impl IntoIterator<Item = (&'static str, String)>) -> Self {
        self.links.get_or_insert(HashMap::default()).extend(links);
        self
    }
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
