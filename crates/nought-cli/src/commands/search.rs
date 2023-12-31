use clap::Error;
use colored::Colorize;
use nought::models::search_models::PackageResponse;
use nought::networks::search_apis::search_package_by_name;
use crate::errors::common_errors::{deserialize_error, network_error};

pub(crate) async fn _search(api_source: String, pkg_name: &String) -> Result<(), Error> {
    let packages: Vec<PackageResponse> = search_package_by_name(api_source, pkg_name)
        .await
        .map_err(|_| network_error())?
        .json()
        .await
        .map_err(|_| deserialize_error())?;
    packages.iter().for_each(|package| {
        let package = package.clone();
        println!("{}",
                 search_entry(
                     package.repo_name,
                     package.pkg_name,
                     package.pkg_version,
                     package.pkg_description.unwrap_or("".to_string())
                 )
        )
    });

    Ok(())
}

fn search_entry(
    repo_name: String,
    pkg_name: String,
    pkg_version: String,
    description: String
) -> String {
    format!("{}/{}  {} \n {}",
            repo_name.blue(),
            pkg_name.truecolor(238, 130, 50),
            pkg_version,
            description
    )
}