//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern,
// mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);

// Personal preference.
pub use std::format as f;
