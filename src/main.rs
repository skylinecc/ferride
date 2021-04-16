use gtk::prelude::*;
use std::env::args;
use gtk::glib::clone;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod greeter;
mod window;
mod info;
mod project;

fn main() {
    pretty_env_logger::init();

    let application = gtk::Application::new(Some("org.skylinecc.Ferride"), Default::default())
        .expect("Initialization failed...");

    let resources_bytes = include_bytes!("resources/resources.gresource");
    let resource_data = gtk::glib::Bytes::from(&resources_bytes[..]);
    let res = gtk::gio::Resource::from_data(&resource_data).unwrap();
    gtk::gio::resources_register(&res);

    application.connect_activate(clone!(@strong application => move |_| {
        greeter::GreeterWindow::build_ui(&application);
    }));

    application.run(&args().collect::<Vec<_>>());
}
