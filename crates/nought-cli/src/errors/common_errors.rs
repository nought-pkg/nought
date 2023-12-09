use crate::commands_config::Args;
use clap::error::ErrorKind;
use clap::{CommandFactory, Error};

pub(crate) fn network_error() -> Error {
    Args::command().error(ErrorKind::Io, "Network error.")
}

pub(crate) fn deserialize_error() -> Error {
    Args::command().error(ErrorKind::Io, "Deserialize JSON response failed.")
}

pub(crate) fn repos_file_read_error() -> Error {
    Args::command().error(ErrorKind::Io, "Repositories file read error.")
}
