use clap::error::ErrorFormatter;
use crate::commands::search::_search;

mod install;
mod search;

#[inline]
pub(crate) fn install(pkg_name: String) {
}

#[inline]
pub(crate) fn search(pkg_name: String) {
    _search(pkg_name)
}

#[inline]
pub(crate) fn upgrade(pkg_name: Option<String>) {

}

#[inline]
pub(crate) fn sync() {
}