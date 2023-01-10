mod composer;
mod helpers;
mod npm;
mod types;

use crate::composer::{insert_composer_data_in_to_csv, read_composer_json};
use crate::npm::{insert_package_json_data_in_to_csv, read_package_json};

use csv::Writer;
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
