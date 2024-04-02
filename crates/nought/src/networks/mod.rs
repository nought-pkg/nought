use std::error::Error;

pub mod create_api;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;