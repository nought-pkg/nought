use crate::commands_config::Args;
use clap::error::ErrorKind;
use clap::{CommandFactory, Error};

pub(crate) fn network_error() -> Error {
    Args::command().error(ErrorKind::Io, "Network error.")
}
