use serde::Deserialize;
use toml::Table;
use crate::protos::platforms::Platforms;

#[derive(Deserialize)]
pub struct Package {
    pub package: PackageInfo,
    pub dependencies: Table
}

#[derive(Deserialize)]
pub struct PackageInfo {
    pub pkg_name: String,
    pub pkg_version: String,
    pub pkg_description: Option<String>,
    pub platform: Platforms,
    pub url: Option<String>,
}