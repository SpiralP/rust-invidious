pub mod api;
mod error;

pub use crate::error::{Error, ErrorKind};

pub const API_URL: &str = "https://invidious.snopyta.org/api/v1";
