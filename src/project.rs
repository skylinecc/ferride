use std::path::PathBuf;
use thiserror::Error;
use cargo_toml::Manifest;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("{0}")]
    Manifest(String)
}

pub struct Project {
    pub path: PathBuf,
    pub manifest: Manifest,
}

impl Project {
    pub fn new(path: &PathBuf) -> Result<Self, ProjectError> {
        let path = path.to_owned();

        let mut manifest = match Manifest::from_path(path.clone()) {
            Ok(data) => data,
            Err(error) => return Err(ProjectError::Manifest(error.to_string())),
        };

        match manifest.complete_from_path(&path) {
            Ok(()) => (),
            Err(error) => return Err(ProjectError::Manifest(error.to_string())),
        };

        return Ok(Self {
            path,
            manifest,
        });
    }
}