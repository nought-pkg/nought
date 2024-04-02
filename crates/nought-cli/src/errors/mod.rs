use clap::{CommandFactory, Error};
use clap::error::ErrorKind;
use crate::commands_config::Args;

pub fn build_error(kind: ErrorKind, message: String) -> Error {
    Args::command().error(kind, message)
}