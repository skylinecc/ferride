mod imp;

use glib::Object;
use gtk::{glib, glib::clone, prelude::GtkWindowExt};
use gtk::prelude::*;

use crate::project::Project;
use crate::window::IdeWindow;

glib::wrapper! {
    pub struct WelcomeWindow(ObjectSubclass<imp::WelcomeWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow;
}

impl WelcomeWindow {
    pub fn new(application: &gtk::Application) -> Self {
        debug!("Starting WelcomeWindow");

        let window: Self = Object::new(&[("application", application)]).expect("Failed to create WelcomeWindow");

        // window properties
        window.set_title(Some("Ferride - Welcome"));
        window.set_default_size(350, 450);
        window.set_resizable(false);

        let container = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .spacing(6)
            .vexpand(true)
            // .hexpand(true)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(6)
            .margin_end(6)
            .build();
        
        window.set_child(Some(&container));

        // Title Box
        let title_box = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Start)
            .spacing(10)
            .build();

        container.append(&title_box);

        let logo = gtk::ImageBuilder::new()
            .resource("/org/skylinecc/Ferride/ferris_ferride.svg")
            .pixel_size(110)
            .build();

        title_box.append(&logo);

        let title = gtk::LabelBuilder::new()
            .label("Ferride")
            .css_classes(vec!["large-title".to_string()])
            .build();
        
        title_box.append(&title);

        let subtitle = gtk::LabelBuilder::new()
            .label("<i>A Simple, Cross Platform IDE for Rust.</i>")
            .use_markup(true)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .build();

        container.append(&subtitle);

        let open_button = gtk::ButtonBuilder::new()
            .valign(gtk::Align::End)
            .vexpand(true)
            .label("Open Project")
            .build();

        open_button.connect_clicked(clone!(@weak window, @weak application => move |_| {
            window.open_project(&application);
        }));

        container.append(&open_button);

        return window;
    }

    fn open_project(&self, application: &gtk::Application) {
        info!("Opening project...");

        let cargo_filter = gtk::FileFilter::new();
        cargo_filter.add_pattern("Cargo.toml");

        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open Cargo.toml"),
            Some(self),
            gtk::FileChooserAction::Open,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );
        file_chooser.set_transient_for(Some(self));
        file_chooser.add_filter(&cargo_filter);

        file_chooser.connect_response(clone!(@weak application, @weak self as s => move |d: &gtk::FileChooserDialog, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Ok {
                let file = match d.file() {
                    Some(data) => data,
                    None => {
                        s.error("Couldn't get file", "bug prob tbh");
                        std::process::exit(1);
                    },
                };

                let filename = match file.path() {
                    Some(data) => data,
                    None => {
                        s.error("Couldn't get path", "bug probably tbh");
                        std::process::exit(1);
                    },
                };

                let project = match Project::new(&filename) {
                    Ok(data) => data,
                    Err(error) => {
                        s.error("Couldn't create Project", &error.to_string());
                        std::process::exit(1);
                    },
                };

                let window = IdeWindow::new(&application, &project);

                d.close();
                s.close();
 
                window.show();
            } else if response == gtk::ResponseType::Cancel {
                d.close();
            };
        }));

        file_chooser.show();
    }

    pub fn error<T: AsRef<str>>(&self, main: T, minor: T) {
        let dialog = gtk::MessageDialogBuilder::new()
            .buttons(gtk::ButtonsType::Close)
            .transient_for(self)
            .text(main.as_ref())
            .secondary_text(minor.as_ref())
            .title("Ferride Error")
            .can_focus(true)
            .destroy_with_parent(true)
            .message_type(gtk::MessageType::Error)
            .build();

        dialog.connect_response(clone!(@weak self as s => move |d: &gtk::MessageDialog, _: gtk::ResponseType| {
            d.close();
        }));

        dialog.show();
    }
}
