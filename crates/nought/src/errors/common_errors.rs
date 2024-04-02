use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct NetworkError;

impl Error for NetworkError {
}

impl Display for NetworkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Network error.")
    }
}