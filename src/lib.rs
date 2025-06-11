pub mod signal;
pub mod blocks;
pub mod engine;
pub mod error;

#[cfg(feature = "editor")]
pub mod editor;

pub use error::{PlcError, Result};
