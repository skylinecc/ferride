use gtk::prelude::*;
use std::env::args;

mod window;

fn main() {
    let application =
        gtk::Application::new(Some("org.skylinecc.Ride"), Default::default())
            .expect("Initialization failed...");

    let resources_bytes = include_bytes!("resources/resources.gresource");
    let resource_data = gtk::glib::Bytes::from(&resources_bytes[..]);
    let res = gtk::gio::Resource::from_data(&resource_data).unwrap();
    gtk::gio::resources_register(&res);

    application.connect_activate(|app| {
        let welcome = window::WelcomeWindow::build_ui(app);
        app.add_window(&welcome.window);

        welcome.window.present();
    });

    application.run(&args().collect::<Vec<_>>());
}
