use csv::Writer;
use std::{collections::HashMap, fs::File};

mod types;

use crate::types::{
    ComposerPackage, PackageRow, PackagistResponse, PackagistVersion, VersionNotFound,
};

const RESULT_FILE_PATH: &str = "/home/gadsdev/projects/rust/generate-libs-docs/files/result.csv";
const COMPOSER_FILE_PATH: &str =
    "/home/gadsdev/projects/rust/generate-libs-docs/files/composer.json";

#[tokio::main]
async fn main() {
    let composer_json = read_composer_json(COMPOSER_FILE_PATH);

    let mut result_file = Writer::from_path(RESULT_FILE_PATH).unwrap();

    insert_composer_data_in_to_csv(composer_json.require, &mut result_file).await;
    insert_composer_data_in_to_csv(composer_json.require_dev, &mut result_file).await;
}

fn read_composer_json(file_path: &str) -> ComposerPackage {
    let string_file = std::fs::read_to_string(file_path).unwrap();

    serde_json::from_str::<ComposerPackage>(&string_file).unwrap()
}

async fn insert_composer_data_in_to_csv(
    values: HashMap<String, String>,
    csv_file: &mut Writer<File>,
) {
    for (key, value) in values {
        let package_name = key.to_string();
        if key != "php" && !key.contains("ext") && key != "enlightn/enlightnpro" {
            let package_manager_info = search_composer_package(&package_name).await;

            let version_info = search_valid_version(&value, &package_manager_info.package.versions)
                .expect("Invalid versionFailed to get version");

            let package_row =
                composer_package_row(&package_manager_info, version_info, &package_name);
            csv_file.serialize(package_row).unwrap();
        }
    }
}

// TODO Recatory to parallel requests, first get all values formatted next inser in to csv

async fn search_composer_package(package_name: &String) -> PackagistResponse {
    let url = format!("https://repo.packagist.org/packages/{}.json", package_name);

    let res = reqwest::get(url)
        .await
        .expect("[ERROR] -> Failed to get current price");

    res.json::<PackagistResponse>()
        .await
        .expect("[ERROR] -> Failed to parse to json")
}

fn search_valid_version<'a>(
    composer_version: &String,
    versions: &'a HashMap<String, PackagistVersion>,
) -> Result<&'a PackagistVersion, VersionNotFound> {
    let mut clean_version = composer_version.replace("v", "");
    clean_version = clean_version.replace("^", "");
    clean_version = clean_version.replace("~", "");
    clean_version = format!("v{}", &clean_version);

    // Search with v
    if let Some(copyright_info) = versions.get(&clean_version) {
        return Ok(&copyright_info);
    }

    clean_version = clean_version.replace("v", "");

    // search with none key
    if let Some(copyright_info) = versions.get(&clean_version) {
        return Ok(&copyright_info);
    }

    clean_version = format!("{}.0", &clean_version);

    // search with none key add number
    if let Some(copyright_info) = versions.get(&clean_version) {
        return Ok(&copyright_info);
    }

    // Serach with v and add number
    clean_version = format!("v{}", &clean_version);

    // search with none key add number
    if let Some(copyright_info) = versions.get(&clean_version) {
        return Ok(&copyright_info);
    }

    Err(VersionNotFound {
        message: String::from("Version not found"),
    })
}

fn composer_package_row(
    package_manager_info: &PackagistResponse,
    version_info: &PackagistVersion,
    package_name: &String,
) -> PackageRow {
    let license = version_info
        .license
        .clone()
        .into_iter()
        .nth(0)
        .expect("Not found license");

    PackageRow {
        name: package_manager_info.package.name.to_string(),
        description: package_manager_info.package.description.to_string(),
        copyright: license.to_string(),
        license: license,
        version: version_info.version.to_string(),
        impementation_description: String::from("installed with composer at project"),
        path: format!("/vendor/{}", package_name),
        reference: package_manager_info.package.repository.to_string(),
        language: package_manager_info.package.language.to_string(),
        install: String::from("composer"),
    }
}
