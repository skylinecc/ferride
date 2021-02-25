use std::path::PathBuf;
use cargo_toml::Manifest;

pub fn get_name(path: PathBuf) -> Result<String, String> {
    let manifest = match Manifest::from_path(path.as_path()) {
        Ok(data) => data,
        Err(error) => {
            return Err("invalid toml".to_string());
        },
    };

    let package = match manifest.package {
        Some(data) => data,
        None => return Err("invalid package".to_string()),
    };

    return Ok(package.name);
}
