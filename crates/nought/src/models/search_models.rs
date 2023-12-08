use serde::Deserialize;
use crate::protos::platforms::Platforms;

#[derive(Deserialize, Clone)]
pub struct PackageResponse {
    pub repo_name: String,
    pub pkg_name: String,
    pub pkg_version: String,
    pub pkg_description: Option<String>,
    pub platform: Platforms,
    pub url: Option<String>,
}