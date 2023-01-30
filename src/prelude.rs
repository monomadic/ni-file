//! Crate prelude

pub use crate::error::NIFileError;

// Generic Wrapper tuple struct for newtype pattern,
// mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);

pub use std::format as f;
