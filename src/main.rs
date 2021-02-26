use gtk::prelude::*;
use std::env::args;
use gtk::glib::clone;
use std::process::Command;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod welcome;
mod window;
mod project;
mod info;

fn main() {
    pretty_env_logger::init();

    let application = gtk::Application::new(Some("org.skylinecc.Ferride"), Default::default())
        .expect("Initialization failed...");

    let resources_bytes = include_bytes!("resources/resources.gresource");
    let resource_data = gtk::glib::Bytes::from(&resources_bytes[..]);
    let res = gtk::gio::Resource::from_data(&resource_data).unwrap();
    gtk::gio::resources_register(&res);

    application.connect_activate(clone!(@strong application => move |_| {
        welcome::WelcomeWindow::build_ui(&application);
    }));

    application.run(&args().collect::<Vec<_>>());
}
