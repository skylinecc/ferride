use std::process::Command;

// Returning a blank string as an error is not the proper way to do it but it works anyway.
pub fn rustc_version() -> Result<String, String> {
    match Command::new("rustc")
        .arg("--version")
        .output() {
        Ok(data) => {
            match std::str::from_utf8(&data.stdout) {
                Ok(stdout) => return Ok(stdout.to_string()),
                Err(_) => return Err(String::new()),
            };
        },
        Err(_) => return Err(String::new()),
    };
}
