//! All the types specific to the tracke.rs domain
//!
//! Types present in the [types](crate::types) module are shared between models
//! present in the crate. Besides the type definitions itself conversions and
//! json schemas for the types are defined within the module's sources as well.
//! Unlike models the types defined here have usually just one field or they
//! use the newtype pattern.

pub mod duration;
pub use duration::Duration;
pub mod tags;
pub use tags::Tags;
pub mod deadline_time;
pub use deadline_time::DeadlineTime;
pub mod string;
pub use string::String;
