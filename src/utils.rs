use gtk::prelude::*;
use colored::Colorize;
use clap::ArgMatches;
use std::path::{Path, PathBuf};

pub fn error_dialog<T: AsRef<str>>(text: T) {
    let window = gtk::Window::new();
    window.set_title(Some("Error"));
    window.set_icon_name(Some("dialog-error"));

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.set_child(Some(&container));

    let icon = gtk::Image::from_icon_name(Some("dialog-error"));
    container.append(&icon);

    let text = gtk::Label::new(Some(text.as_ref()));
    container.append(&text);

    window.show();
}

pub fn parse_cargo_toml_file<T: AsRef<str>>(text: T) -> PathBuf {
    let path = Path::new(text.as_ref());

    if !path.is_file() {
        clap_error("The project configuration must be a file and not a directory");

        std::process::exit(1);
    } else if !path.ends_with("Cargo.toml") {
        clap_error("The project configuration must be named \"Cargo.toml\"");
        std::process::exit(1);
    };

    let pathbuf = match path.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            clap_error("Unable to get path to Cargo.toml");
            std::process::exit(1);
        },
    };

    return pathbuf;
}

pub fn clap_error<T: AsRef<str>>(text: T) {
    eprintln!("{} {}\n", "error:".red().bold(), text.as_ref());
    eprintln!("USAGE:\n    ferride [OPTIONS]\n");
    eprintln!("For more information try {}", "--help".green());
}