use clap::{CommandFactory, Error};
use clap::error::ErrorKind;
use crate::commands_config::Args;

pub(crate) fn network_error() -> Error {
    Args::command()
        .error(
            ErrorKind::Io,
            "Network Error."
        )
}