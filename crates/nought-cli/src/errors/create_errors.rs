use clap::{CommandFactory, Error};
use clap::error::ErrorKind;
use crate::commands_config::Args;

pub(crate) fn unsupported_platform_error() -> Error {
    Args::command().error(ErrorKind::Io, "Unsupported platform.")
}

pub(crate) fn create_core_file_error() -> Error {
    Args::command().error(ErrorKind::Io, "Failed to create server core file.")
}

pub(crate) fn create_config_error() -> Error {
    Args::command().error(ErrorKind::Io, "Failed to create config.")
}

pub(crate) fn dir_already_exist_error() -> Error {
    Args::command().error(ErrorKind::Io, "Directory already existed.")
}