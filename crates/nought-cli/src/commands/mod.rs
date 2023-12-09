use clap::Error;
use crate::commands::search::_search;

mod install;
mod search;

#[inline]
pub(crate) fn install(pkg_name: String) {
    todo!()
}

#[inline]
pub(crate) async fn search(api_sources: Vec<String>, pkg_name: &String) -> Result<(), Error> {
    for api_source in api_sources {
        _search(api_source, pkg_name).await?;
    }
    Ok(())
}

#[inline]
pub(crate) fn upgrade(pkg_name: Option<String>) {
    todo!()
}

#[inline]
pub(crate) fn sync() {
    todo!()
}