//! CodeSage Core Library
//!
//! This crate provides the foundational types and traits used throughout CodeSage.

pub mod error;
pub mod models;
pub mod traits;

pub use error::{CodeSageError, Result};
pub use models::*;
pub use traits::*;
