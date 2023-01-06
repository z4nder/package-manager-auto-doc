
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use csv::Writer;

#[derive(Deserialize, Debug)]
struct Package {
    require: HashMap<String, String>,
    #[serde(rename = "require-dev")]
    require_dev: HashMap<String, String>
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
#[warn(unused_must_use)]
fn main() {
    let file = read_file();   
   
    let mut result_file = Writer::from_path("/home/gadsdev/projects/rust/generate-libs-docs/files/result.csv").unwrap();

    for (key, value) in file.require {
        let test = key.to_string();
        if value != "*" || key != "php"{
            result_file.serialize(PackageDoc {
                name: key,
                description: String::from(""),
                copyright: String::from(""),
                license: String::from(""),
                version: value.replace("^", ""),
                impementation_description: String::from("installed with composer at project"),
                path: String::from(""),
                reference: format!("/vendor/{}", &test),
                language: String::from("php"),
                install: String::from("composer")
            }).unwrap();
        }
    } 
    
    for (key, value) in file.require_dev {
       
        result_file.serialize(PackageDoc {
            name: key,
            description: String::from(""),
            copyright: String::from(""),
            license: String::from(""),
            version: value,
            impementation_description: String::from(""),
            path: String::from(""),
            reference: String::from(""),
            language: String::from("PHP"),
            install: String::from("composer")
        }).unwrap();
    } 

    // dbg!("{:#?}",  result_file);
    
}

fn read_file() -> Package{    
    let file_path = "/home/gadsdev/projects/rust/generate-libs-docs/files/composer.json";
    let string_file = std::fs::read_to_string(&file_path).unwrap();

    serde_json::from_str::<Package>(&string_file).unwrap() 
}
