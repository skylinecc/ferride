use std::process::Command;

fn main() {
    let _ = Command::new("sh")
        .args(&[
            "-c",
            "cd src/resources && glib-compile-resources resources.xml",
        ])
        .output()
        .expect("failed to execute glib-compile-resources");
}
