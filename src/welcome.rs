use gtk::glib::clone;
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

        let window_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        window.set_child(Some(&window_box));
        window_box.set_margin_end(12);
        window_box.set_margin_start(12);
        window_box.set_margin_top(12);
        window_box.set_margin_bottom(12);

        // Set all the stupid boilerplate code...
        let title_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        title_box.set_halign(gtk::Align::Center);

        let title_label = gtk::Label::new(None);
        title_label.set_markup(&format!(
            "<span weight=\"bold\" size=\"xx-large\">Ride</span>"
        ));

        let title_image = gtk::Image::from_resource("/org/skylinecc/Ride/rust_logo.svg");
        title_image.set_size_request(50, 50);

        title_box.append(&title_label);
        title_box.append(&title_image);
        window_box.append(&title_box);

        let subtitle_label = gtk::Label::new(None);
        subtitle_label.set_markup(&format!(
            "<span weight=\"light\" size=\"small\">A simple Rust IDE</span>"
        ));
        window_box.append(&subtitle_label);

        let dummy_label = gtk::Label::new(None);
        dummy_label.set_vexpand(true);
        window_box.append(&dummy_label);

        let open_button = gtk::Button::with_label("Open a Project");
        window_box.append(&open_button);
        open_button.set_halign(gtk::Align::Center);

        let link_box = gtk::Box::new(gtk::Orientation::Horizontal, 32);
        link_box.set_halign(gtk::Align::Center);
        window_box.append(&link_box);

        let homepage_link = gtk::Label::new(None);
        homepage_link.set_markup(&format!(
            "<a href=\"https://skylinecc.github.io/ride/\" title=\"Website\">Homepage</a>"
        ));
        homepage_link.set_halign(gtk::Align::Center);
        link_box.append(&homepage_link);

        let repository_link = gtk::Label::new(None);
        repository_link.set_markup(&format!(
            "<a href=\"https://github.com/skylinecc/ride/\" title=\"Repository\">Source Code</a>"
        ));
        repository_link.set_halign(gtk::Align::Center);
        link_box.append(&repository_link);

        let authors_link = gtk::Label::new(None);
        authors_link.set_markup(&format!("<a href=\"https://github.com/skylinecc/ride/graphs/contributors\" title=\"Contributors\">Authors</a>"));
        authors_link.set_halign(gtk::Align::Center);
        link_box.append(&authors_link);

        // Open Project
        open_button.connect_clicked(clone!(@strong window => move |_| {

        }));

        Self { window }
    }

    pub fn open_project(window: gtk::ApplicationWindow) -> std::path::PathBuf {
        println!("Opening Project...");

        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open Cargo.toml"),
            Some(&window),
            gtk::FileChooserAction::Open,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );

        let cargo_filter = gtk::FileFilter::new();
        cargo_filter.set_name(Some("Cargo Project File"));
        cargo_filter.add_pattern("Cargo.toml");
        file_chooser.add_filter(&cargo_filter);

        file_chooser.connect_response(move |d: &gtk::FileChooserDialog, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Ok {
                let file = d.get_file().expect("Couldn't get project file");
                let path = file.get_path().expect("Error getting project path");

                return path;
            };

            d.close();
        });

        file_chooser.show();
    }
}
