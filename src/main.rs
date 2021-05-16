// use gtk::prelude::*;
// use std::env::args;
// use gtk::glib::clone;
// use gtk::glib;





// mod welcome;

// fn main() {
//     pretty_env_logger::init();

//     let matches = app_from_crate!()
//         .arg(
//             Arg::with_name("directory")
//                 .short("d")
//                 .long("directory")
//                 .value_name("DIRECTORY")
//                 .help("Sets a custom project directory")
//                 .takes_value(true)
//         )
//         .get_matches();

//     let application =
//         gtk::Application::new(Some(APP_ID), Default::default());

//     let resources_bytes = include_bytes!("resources/resources.gresource");
//     let resource_data = gtk::glib::Bytes::from(&resources_bytes[..]);
//     let res = gtk::gio::Resource::from_data(&resource_data).unwrap();
//     gtk::gio::resources_register(&res);

//     application.connect_activate(clone!(@strong matches => move |application| {
//         activate(application, matches);
//     }));

//     application.run();
// }

// fn activate(application: &gtk::Application, matches: ArgMatches) {
//     let window = gtk::ApplicationWindow::new(application);

//     window.set_title(Some("First GTK Program"));
//     window.set_default_size(350, 70);

//     if matches.is_present("DIRECTORY") {
//         println!("you had a directory tag: {}", matches.value_of("DIRECTORY").unwrap());
//     };

//     let button = gtk::Button::with_label("Press me!");

//     window.set_child(Some(&button));

//     window.show();
// }
mod utils;
mod welcome;

use utils::{clap_error, parse_cargo_toml_file};

use gtk::prelude::*;
use gtk::glib;
use glib::clone;

use clap::{Arg, ArgMatches, app_from_crate, crate_description, crate_version, crate_authors, crate_name};

extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub const APP_DIR: &str = "/org/skylinecc/Ferride";
pub const APP_ID: &str = "org.skylinecc.Ferride";

fn main() {
    pretty_env_logger::init();
    debug!("Welcome to Ferride!");

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

    let application = gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default()).unwrap();

    application.connect_activate(move |application| {
        build_ui(application, matches.clone());
    });

    application.run(&[]);
}

fn build_ui(application: &gtk::Application, matches: ArgMatches) {
    trace!("main::build_ui");

    match matches.value_of("project") {
        Some(path) => {
            let path = parse_cargo_toml_file(path);

            let window = gtk::ApplicationWindow::new(application);

            window.set_title(Some(&path.to_string_lossy()));
            window.set_default_size(350, 70);
        
        
            let button = gtk::Button::with_label("Click me!");
        
            window.set_child(Some(&button));
        
            window.show();
        },
        None => {

        },
    };
}