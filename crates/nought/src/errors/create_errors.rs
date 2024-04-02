use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct VersionNotFound;

impl Error for VersionNotFound {
}

impl Display for VersionNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Given version for server core is not found.")
    }
}

#[derive(Debug)]
pub struct CreateServerDirectoryFailed;

impl Error for CreateServerDirectoryFailed {
}

impl Display for CreateServerDirectoryFailed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Permission denied or path does not exist.")
    }
}

#[derive(Debug)]
pub struct ConfigSerializeFailed;

impl Error for ConfigSerializeFailed {
}

impl Display for ConfigSerializeFailed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Permission denied or path does not exist.")
    }
}

