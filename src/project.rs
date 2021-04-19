use std::path::PathBuf;
use cargo_toml::Manifest;

#[derive(Clone, PartialEq)]
pub struct Project{
    pub name: String,
    pub path: PathBuf,
    pub manifest: Manifest,
}

impl Project {
    pub fn new(path: &PathBuf) -> Result<Self, (String, String)> {
        let manifest = match Manifest::from_path(path) {
            Ok(data) => data,
            Err(error) => {
                return Err(("Couldn't get Cargo.toml manifest".to_string(), error.to_string()));
            },
        };

        let package = match manifest.clone().package {
            Some(package) => package,
            None => {
                return Err(("Malformed Cargo.toml".to_string(), "Couldn't find package section in manifest.".to_string()));
            }
        };

        let name = package.name;
        
        Ok(Self {
            name,
            path: path.to_owned(),
            manifest,
        })
    }
}