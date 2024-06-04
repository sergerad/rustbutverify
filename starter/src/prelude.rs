//! Crate prelude

// Re-export error type
pub use crate::error::Error;

// Result type using crate's error type
pub type Result<T> = core::result::Result<T, Error>;

// Generic wrapper tuple struct for newtype pattern.
// Allows us to impl external trait on external type.
pub struct NewType<T>(pub T);
