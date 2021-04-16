use std::path::PathBuf;
use cargo_toml::Manifest;

use crate::info::gtk_error;

#[derive(Clone, PartialEq)]
pub struct Project{
    pub name: String,
    pub path: PathBuf,
    pub manifest: Manifest,
}

impl Project {
    pub fn new(path: &PathBuf) -> Self {
        let manifest = match Manifest::from_path(path) {
            Ok(data) => data,
            Err(error) => {
                gtk_error("Couldn't get Cargo.toml manifest", error.to_string().as_str(), None);
                std::process::exit(1);
            },
        };

        let package = match manifest.clone().package {
            Some(package) => package,
            None => {
                gtk_error("Malformed Cargo.toml", "Couldn't find package section in manifest.", None);
                std::process::exit(1);
            }
        };

        let name = package.name;
        
        Self {
            name,
            path: path.to_owned(),
            manifest,
        }
    }
}