use gtk::prelude::*;

pub struct WelcomeWindow {
    window: gtk::ApplicationWindow,
    builder: gtk::Builder,
}

impl WelcomeWindow {
    pub fn build_ui(application: &gtk::Application) -> Self {
        let builder = gtk::Builder::from_resource("/org/skylinecc/Ride/welcome.ui");
        let window: gtk::ApplicationWindow = builder.get_object("welcome_window").expect("Couldn't get Welcome Window!");

        window.show();

        Self {
            window,
            builder
        }

    }
}
