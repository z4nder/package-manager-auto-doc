use csv::Writer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Package {
    require: HashMap<String, String>,
    #[serde(rename = "require-dev")]
    require_dev: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PackageDoc {
    name: String,
    description: String,
    copyright: String,
    license: String,
    version: String,
    impementation_description: String,
    path: String,
    reference: String,
    language: String,
    install: String,
}

#[derive(Deserialize, Debug)]
struct PackagistPackage {
    package: PackagistInfo,
}
#[derive(Deserialize, Debug)]
struct PackagistInfo {
    name: String,
    description: String,
    repository: String,
    language: String,
    versions: HashMap<String, PackagistVersion>,
}
#[derive(Deserialize, Debug, Clone)]
struct PackagistVersion {
    version: String,
    license: Vec<String>,
}
#[derive(Debug)]
struct VersionNotFound {
    message: String,
}

#[tokio::main]
async fn main() {
    let file = read_file();

    let mut result_file =
        Writer::from_path("/home/gadsdev/projects/rust/generate-libs-docs/files/result.csv")
            .unwrap();

    for (key, value) in file.require {
        let package_name = key.to_string();
        if key != "php" && !key.contains("ext") {
            let package_manager_info = get_package_info(&package_name).await;

            //TODO Create function at string to get that info
            let version_info =
                search_valid_version(&value, &package_manager_info.package.versions).unwrap();

            result_file
                .serialize(PackageDoc {
                    name: package_manager_info.package.name,
                    description: package_manager_info.package.description,
                    copyright: version_info.to_string(),
                    license: version_info,
                    version: value.replace("^", ""),
                    impementation_description: String::from("installed with composer at project"),
                    path: format!("/vendor/{}", &package_name),
                    reference: package_manager_info.package.repository,
                    language: package_manager_info.package.language,
                    install: String::from("composer"),
                })
                .unwrap();
        }
    }

    for (key, value) in file.require_dev {
        let package_name = key.to_string();
        if key != "php" && !key.contains("ext") && key != "enlightn/enlightnpro" {
            let package_manager_info = get_package_info(&package_name).await;

            //TODO Create function at string to get that info
            let version_info = search_valid_version(&value, &package_manager_info.package.versions)
                .expect("Invalid versionFailed to get version");

            result_file
                .serialize(PackageDoc {
                    name: package_manager_info.package.name,
                    description: package_manager_info.package.description,
                    copyright: version_info.to_string(),
                    license: version_info,
                    version: value.replace("^", ""),
                    impementation_description: String::from("installed with composer at project"),
                    path: format!("/vendor/{}", &package_name),
                    reference: package_manager_info.package.repository,
                    language: package_manager_info.package.language,
                    install: String::from("composer"),
                })
                .unwrap();
        }
    }

    // dbg!("{:#?}",  result_file);
}

fn read_file() -> Package {
    let file_path = "/home/gadsdev/projects/rust/generate-libs-docs/files/composer.json";
    let string_file = std::fs::read_to_string(&file_path).unwrap();

    serde_json::from_str::<Package>(&string_file).unwrap()
}

async fn get_package_info(package_name: &String) -> PackagistPackage {
    let url = format!("https://repo.packagist.org/packages/{}.json", package_name);

    let res = reqwest::get(url)
        .await
        .expect("[ERROR] -> Failed to get current price");

    res.json::<PackagistPackage>()
        .await
        .expect("[ERROR] -> Failed to parse to json")
}

fn search_valid_version(
    composer_version: &String,
    versions: &HashMap<String, PackagistVersion>,
) -> Result<String, VersionNotFound> {
    let mut clean_version = composer_version.replace("v", "");
    clean_version = clean_version.replace("^", "");
    clean_version = clean_version.replace("~", "");
    clean_version = format!("v{}", &clean_version);

    // Search with v
    if let Some(copyright_info) = versions.get(&clean_version) {
        return Ok(copyright_info.license.clone().into_iter().nth(0).unwrap());
    }

    clean_version = clean_version.replace("v", "");

    // search with none key
    if let Some(copyright_info) = versions.get(&clean_version) {
        return Ok(copyright_info.license.clone().into_iter().nth(0).unwrap());
    }

    clean_version = format!("{}.0", &clean_version);

    // search with none key add number
    if let Some(copyright_info) = versions.get(&clean_version) {
        return Ok(copyright_info.license.clone().into_iter().nth(0).unwrap());
    }

    // Serach with v and add number
    clean_version = format!("v{}", &clean_version);

    // search with none key add number
    if let Some(copyright_info) = versions.get(&clean_version) {
        return Ok(copyright_info.license.clone().into_iter().nth(0).unwrap());
    }

    Err(VersionNotFound {
        message: String::from("Version not found"),
    })
}
