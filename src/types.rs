use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ComposerPackage {
    pub require: HashMap<String, String>,
    #[serde(rename = "require-dev")]
    pub require_dev: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageRow {
    pub name: String,
    pub description: String,
    pub copyright: String,
    pub license: String,
    pub version: String,
    pub reference: String,
    pub language: String,
    pub install: String,
}

#[derive(Deserialize, Debug)]
pub struct PackagistResponse {
    pub package: PackagistInfo,
}

#[derive(Deserialize, Debug)]
pub struct PackagistInfo {
    pub name: String,
    pub description: String,
    pub repository: String,
    pub language: String,
    pub versions: HashMap<String, PackagistVersion>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PackagistVersion {
    pub version: String,
    pub license: Vec<String>,
}

#[derive(Debug)]
pub struct VersionNotFound {
    pub message: String,
}

// NPM
#[derive(Deserialize, Debug)]
pub struct PackageJson {
    pub dependencies: HashMap<String, String>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
}
#[derive(Deserialize, Debug)]
pub struct NpmResponse {
    pub collected: NpmCollected,
}
#[derive(Deserialize, Debug)]
pub struct NpmCollected {
    pub metadata: NpmMetaData,
}
#[derive(Deserialize, Debug)]
pub struct NpmMetaData {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub links: NpmLinks,
    pub license: String,
}

#[derive(Deserialize, Debug)]
pub struct NpmLinks {
    pub npm: String,
    pub repository: Option<String>,
}
