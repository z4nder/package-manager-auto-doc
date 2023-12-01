use csv::Writer;
use std::{collections::HashMap, fs::File};
use indicatif::ProgressBar;

use crate::helpers::extract_copyright_from_github;
use crate::types::{NpmMetaData, NpmResponse, PackageJson, PackageRow};

pub async fn insert_package_json_data_in_to_csv(
    values: HashMap<String, String>,
    csv_file: &mut Writer<File>,
    bar: &mut ProgressBar,
    used_in_prod: bool,
) {
    for (key, value) in values {
        let package_name = clear_package_json_package_name(key);

        let package_manager_info: NpmResponse = search_npm_package(package_name.clone()).await;

        let github_link = match &package_manager_info.collected.metadata.links.repository {
            None => String::from("empty"),
            Some(value) => value.to_string(),
        };
        let mut copyright = String::from("Not Found");

        if github_link != "empty" {
            copyright = extract_copyright_from_github(&github_link).await;
        }

        let package_row = package_json_row(
            &package_manager_info.collected.metadata,
            value,
            package_name,
            copyright,
            used_in_prod
        );

        csv_file.serialize(package_row).unwrap();

        bar.inc(1);
    }
}

pub fn read_package_json(file_path: &str) -> PackageJson {
    let string_file = std::fs::read_to_string(file_path).unwrap();

    serde_json::from_str::<PackageJson>(&string_file).unwrap()
}

fn clear_package_json_package_name(package_name: String) -> String {
    if package_name.contains("/") {
        return package_name
            .split("/")
            .last()
            .expect("Failed to parse package name")
            .to_string();
    }

    package_name
}

// TODO Recatory to parallel requests, first get all values formatted next inser in to csv
async fn search_npm_package(package_name: String) -> NpmResponse {
    let url = format!("https://api.npms.io/v2/package/{}", package_name);

    let res = reqwest::get(url)
        .await
        .expect("[ERROR] -> Failed to get current package");

    res.json::<NpmResponse>()
        .await
        .expect("[ERROR] -> Failed to parse to json")
}

fn package_json_row(
    package_manager_info: &NpmMetaData,
    version: String,
    package_name: String,
    copyright: String,
    used_in_prod: bool
) -> PackageRow {
    PackageRow {
        name: package_manager_info.name.to_string(),
        description: match &package_manager_info.description {
            None => String::from("empty"),
            Some(description) => description.to_string(),
        },
        copyright: copyright,
        license: package_manager_info.license.to_string(),
        version: version,
        path: format!("/{}/{}", "node_modules", package_name),
        reference: package_manager_info.links.npm.to_string(),
        language: String::from("JavaScript"),
        install: String::from("npm"),
        used_in_prod: used_in_prod.to_string().to_uppercase(),
    }
}
