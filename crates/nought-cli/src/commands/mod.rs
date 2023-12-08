use crate::commands::search::_search;

mod install;
mod search;

#[inline]
pub(crate) fn install(pkg_name: String) {
}

#[inline]
pub(crate) async fn search(api_sources: Vec<String>, pkg_name: &String) {
    for api_source in api_sources {
        _search(api_source, pkg_name).await;
    }
}

#[inline]
pub(crate) fn upgrade(pkg_name: Option<String>) {

}

#[inline]
pub(crate) fn sync() {
}