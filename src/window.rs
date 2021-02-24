use gtk::prelude::*;

pub struct WelcomeWindow {
    pub window: gtk::ApplicationWindow,
}

impl WelcomeWindow {
    pub fn build_ui(application: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(application);

        window.set_title(Some("Welcome"));
        window.set_default_size(400, 500);
        window.set_resizable(false);

        let window_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        window.set_child(Some(&window_box));

        // Title Box
        let title_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        title_box.set_margin_top(12);
        title_box.set_halign(gtk::Align::Center);

        let title_label = gtk::Label::new(None);
        title_label.set_markup(&format!("<span weight=\"bold\" size=\"xx-large\">Ride</span>"));

        let title_image = gtk::Image::from_resource("/org/skylinecc/Ride/rust_logo.svg");
        title_image.set_size_request(50, 50);

        title_box.append(&title_label);
        title_box.append(&title_image);
        window_box.append(&title_box);

        let subtitle_label = gtk::Label::new(None);
        subtitle_label.set_markup(&format!("<span weight=\"light\" size=\"small\">Ride is a simple Rust IDE</span>"));
        window_box.append(&subtitle_label);


        let open_button = gtk::Button::with_label("Open a Project");
        open_button.set_halign(gtk::Align::Center);
        // open_button.pacl
        window_box.append(&open_button);

        Self {
            window,
        }
    }
}
