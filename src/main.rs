use gtk::prelude::*;
use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("Ride"));
    window.set_default_size(1150, 650);

    let label = gtk::Label::new(Some("Content Here!"));

    window.set_child(Some(&label));

    window.show();
}

fn main() {
    let application =
        gtk::Application::new(Some("org.skylinecc.Ride"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
