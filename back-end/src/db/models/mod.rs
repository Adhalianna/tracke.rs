//! Models of data present in the database.
//!
//! Models are used to map the results of an SQL query to structures
//! understood from the Rust code.

use crate::prelude::*;

/// A dummy table and model used for testing.
#[derive(Queryable, Debug, Serialize, ToSchema)]
pub struct Hello {
    pub key: String,
    pub value: String,
}

impl std::fmt::Display for Hello {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ key: {}, value: {} }}", self.key, self.value)
    }
}
