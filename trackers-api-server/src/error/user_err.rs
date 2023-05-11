use super::*;
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
    pub fn with_docs(mut self) -> Self {
        self.links
            .get_or_insert(HashMap::default())
            .insert("documentation", "/doc".into());
        self
    }
}

impl std::fmt::Display for BadRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for BadRequestError {}

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
    pub fn with_docs(mut self) -> Self {
        self.links
            .get_or_insert(HashMap::default())
            .insert("documentation", "/doc".into());
        self
    }
}

impl std::fmt::Display for UnathorizedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for UnathorizedError {}

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
    pub fn with_docs(mut self) -> Self {
        self.links
            .get_or_insert(HashMap::default())
            .insert("documentation", "/doc".into());
        self
    }
}

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for NotFoundError {}

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
    pub fn with_docs(mut self) -> Self {
        self.links
            .get_or_insert(HashMap::default())
            .insert("documentation", "/doc".into());
        self
    }
}

impl std::fmt::Display for ConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(into = "ApiError")]
pub struct ForbiddenError {
    pub msg: String,
    pub links: Option<HashMap<&'static str, String>>,
}

impl ForbiddenError {
    pub fn with_msg(mut self, msg: impl std::fmt::Display) -> Self {
        self.msg = msg.to_string();
        self
    }
    pub fn with_links(mut self, links: impl IntoIterator<Item = (&'static str, String)>) -> Self {
        self.links.get_or_insert(HashMap::default()).extend(links);
        self
    }
    pub fn with_docs(mut self) -> Self {
        self.links
            .get_or_insert(HashMap::default())
            .insert("documentation", "/doc".into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(into = "ApiError")]
pub struct GoneError {
    pub msg: String,
    pub links: Option<HashMap<&'static str, String>>,
}

impl GoneError {
    pub fn with_msg(mut self, msg: impl std::fmt::Display) -> Self {
        self.msg = msg.to_string();
        self
    }
    pub fn with_links(mut self, links: impl IntoIterator<Item = (&'static str, String)>) -> Self {
        self.links.get_or_insert(HashMap::default()).extend(links);
        self
    }
    pub fn with_docs(mut self) -> Self {
        self.links
            .get_or_insert(HashMap::default())
            .insert("documentation", "/doc".into());
        self
    }
}

impl std::fmt::Display for GoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for GoneError {}

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

impl crate::error::ApiErrorTrait for ForbiddenError {
    fn status(&self) -> u16 {
        403
    }
    fn msg(&self) -> String {
        self.msg.clone()
    }
    fn links(&self) -> &Option<HashMap<&'static str, String>> {
        &self.links
    }
}

impl crate::error::ApiErrorTrait for GoneError {
    fn status(&self) -> u16 {
        410
    }
    fn msg(&self) -> String {
        self.msg.clone()
    }
    fn links(&self) -> &Option<HashMap<&'static str, String>> {
        &self.links
    }
}
