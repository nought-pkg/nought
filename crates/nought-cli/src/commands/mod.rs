use clap::error::ErrorFormatter;
use crate::commands::search::_search;
use crate::errors::common_errors::network_error;

mod install;
mod search;

pub(crate) fn install(pkg_name: String) {
    network_error().exit()
}

pub(crate) fn search(pkg_name: String) {
    _search(pkg_name)
}

pub(crate) fn upgrade(pkg_name: Option<String>) {

}

pub(crate) fn sync() {
}