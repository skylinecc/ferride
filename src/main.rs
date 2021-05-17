mod welcome;
mod window;
mod project;

use welcome::WelcomeWindow;
use window::IdeWindow;
use project::Project;

use std::path::Path;
use gtk::prelude::*;
use clap::{Arg, ArgMatches, app_from_crate, crate_description, crate_version, crate_authors, crate_name};

extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub const APP_DIR: &str = "/org/skylinecc/Ferride/";
pub const APP_ID: &str = "org.skylinecc.Ferride";

fn main() {
    pretty_env_logger::init();
    info!("Welcome to Ferride!");

    let matches = app_from_crate!()
        .arg(
            Arg::with_name("project")
                .short("p")
                .long("project")
                .value_name("Cargo.toml")
                .help("Sets a custom project Cargo.toml configuration to open")
                .takes_value(true)
        )
        .get_matches();

    let resource_data = gtk::glib::Bytes::from(&include_bytes!("resources/resources.gresource")[..]);
    let res = gtk::gio::Resource::from_data(&resource_data).unwrap();
    gtk::gio::resources_register(&res);

    let application = gtk::Application::new(Some("org.skylinecc.Ferride"), Default::default());

    application.connect_activate(move |application| {
        build_ui(application, matches.clone());
    });

    application.run_with_args(&[""]);
}

fn build_ui(application: &gtk::Application, matches: ArgMatches) {
    match matches.value_of("project") {
        Some(path) => {
            let path = Path::new(path);

            if !path.is_file() | !path.ends_with("Cargo.toml") {
                error!("The project configuration must be a file named \"Cargo.toml\"");
                std::process::exit(1);
            };
        
            let pathbuf = match path.canonicalize() {
                Ok(path) => path,
                Err(_) => {
                    error!("Unable to get canonicalized path to Cargo.toml");
                    std::process::exit(1);
                },
            };

            let project = Project::new(&pathbuf);
            let window = IdeWindow::new(application, &project);

            window.show();
        },
        None => {
            let window = WelcomeWindow::new(application);
            window.show();
        },
    };
}