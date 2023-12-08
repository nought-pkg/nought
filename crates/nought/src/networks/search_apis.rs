use reqwest::{Client, Response};

pub async fn search_package_by_name(api_source: String, pkg_name: &String) -> reqwest::Result<Response> {
    Client::new()
        .get(format!("https://{}/packages/{}", api_source, pkg_name))
        .send()
        .await
}