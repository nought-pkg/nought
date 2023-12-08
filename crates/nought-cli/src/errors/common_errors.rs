use clap::{CommandFactory, Error};
use clap::error::ErrorKind;
use crate::commands_config::Args;

pub(crate) fn network_error() -> Error {
    Args::command()
        .error(
            ErrorKind::Io,
            "Network error."
        )
}

pub(crate) fn deserialize_error() -> Error {
    Args::command()
        .error(
            ErrorKind::Io,
            "Deserialize JSON response failed."
        )
}

pub(crate) fn repos_file_read_error() -> Error {
    Args::command()
        .error(
            ErrorKind::Io,
            "Repositories file read error."
        )
}