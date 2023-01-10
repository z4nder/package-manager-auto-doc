use csv::Writer;
use std::{collections::HashMap, fs::File};
mod types;
use crate::types::{
    ComposerPackage, NpmMetaData, NpmResponse, PackageJson, PackageRow, PackagistResponse,
    PackagistVersion, VersionNotFound,
};
use question::{Answer, Question};
use std::fs;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let srcdir = PathBuf::from("./files");
    let absolute_path = fs::canonicalize(&srcdir)
        .expect("Faile to get current path")
        .into_os_string()
        .into_string()
        .unwrap();

    let result_file_path = format!("{}/result.csv", absolute_path);
    let mut result_file = Writer::from_path(result_file_path).unwrap();

    let doc_composer = Question::new("Do you want to document composer.json?")
        .default(Answer::YES)
        .show_defaults()
        .confirm();

    if doc_composer == Answer::YES {
        let compsoer_file_path = format!("{}/composer.json", absolute_path);
        let composer_json = read_composer_json(&compsoer_file_path);
        insert_composer_data_in_to_csv(composer_json.require, &mut result_file).await;
        insert_composer_data_in_to_csv(composer_json.require_dev, &mut result_file).await;
    }

    let doc_package_json = Question::new("Do you want to document package.json?")
        .default(Answer::YES)
        .show_defaults()
        .confirm();

    if doc_package_json == Answer::YES {
        let package_json_file_path = format!("{}/package.json", absolute_path);
        let package_json = read_package_json(&package_json_file_path);
        insert_package_json_data_in_to_csv(package_json.dependencies, &mut result_file).await;
        insert_package_json_data_in_to_csv(package_json.dev_dependencies, &mut result_file).await;
    }
}

fn read_composer_json(file_path: &str) -> ComposerPackage {
    let string_file = std::fs::read_to_string(file_path).unwrap();

    serde_json::from_str::<ComposerPackage>(&string_file).unwrap()
}

fn read_package_json(file_path: &str) -> PackageJson {
    let string_file = std::fs::read_to_string(file_path).unwrap();

    serde_json::from_str::<PackageJson>(&string_file).unwrap()
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

            let copyright =
                extract_copyright_from_github(&package_manager_info.package.repository).await;

            let package_row = composer_package_row(
                &package_manager_info,
                version_info,
                &package_name,
                copyright,
            );
            csv_file.serialize(package_row).unwrap();
        }
    }
}

async fn insert_package_json_data_in_to_csv(
    values: HashMap<String, String>,
    csv_file: &mut Writer<File>,
) {
    for (key, value) in values {
        let package_name = clear_package_json_package_name(key);

        let package_manager_info = search_npm_package(package_name).await;

        let github_link = match &package_manager_info.collected.metadata.links.repository {
            None => String::from("empty"),
            Some(value) => value.to_string(),
        };
        let mut copyright = String::from("Not Found");

        if github_link != "empty" {
            copyright = extract_copyright_from_github(&github_link).await;
        }

        let package_row =
            package_json_row(&package_manager_info.collected.metadata, value, copyright);

        csv_file.serialize(package_row).unwrap();
    }
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

// TODO Recatory to parallel requests, first get all values formatted next inser in to csv
async fn search_composer_package(package_name: &String) -> PackagistResponse {
    let url = format!("https://repo.packagist.org/packages/{}.json", package_name);

    let res = reqwest::get(url)
        .await
        .expect("[ERROR] -> Failed to get current package");

    res.json::<PackagistResponse>()
        .await
        .expect("[ERROR] -> Failed to parse to json")
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
    copyright: String,
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
        copyright: copyright,
        license: license,
        version: version_info.version.to_string(),
        impementation_description: String::from("installed with composer at project"),
        path: format!("/vendor/{}", package_name),
        reference: package_manager_info.package.repository.to_string(),
        language: package_manager_info.package.language.to_string(),
        install: String::from("composer"),
    }
}

fn package_json_row(
    package_manager_info: &NpmMetaData,
    version: String,
    copyright: String,
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
        impementation_description: String::from("installed with npm at project"),
        path: format!("/node_modules/{}", package_manager_info.name),
        reference: package_manager_info.links.npm.to_string(),
        language: String::from("JavaScript"),
        install: String::from("npm"),
    }
}

async fn extract_copyright_from_github(git_url: &String) -> String {
    let mut git_file = git_url.to_string();
    git_file = git_file.replace("github.com/", "raw.githubusercontent.com/");
    git_file = format!("{}/master/LICENSE", &git_file);

    let mut res = reqwest::get(&git_file)
        .await
        .expect("[ERROR] -> Failed to get current package");

    if res.status() == 404 {
        git_file = git_file.replace("LICENSE", "LICENSE.md");
        res = reqwest::get(&git_file)
            .await
            .expect("[ERROR] -> Failed to get current package");
    }

    if res.status() == 404 {
        git_file = git_file.replace("LICENSE.md", "LICENSE.txt");
        res = reqwest::get(&git_file)
            .await
            .expect("[ERROR] -> Failed to get current package");
    }

    let text_response = res
        .text()
        .await
        .expect("[ERROR] -> Failed to parse to json");

    // TODO Case find on Copyright occurence and net is not Copyright stop search to optimize
    //  Look need lowercase reponse string
    let formated: Vec<&str> = text_response.split("\n").collect();
    let formated: Vec<&str> = formated
        .iter()
        .filter(|&element| element.contains("Copyright"))
        .cloned()
        .collect();

    formated.join(", ")
}
