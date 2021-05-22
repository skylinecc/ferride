mod imp;

use glib::Object;
use gtk::{glib, glib::clone, prelude::GtkWindowExt};
use gtk::prelude::*;

use crate::project::Project;

glib::wrapper! {
    pub struct IdeWindow(ObjectSubclass<imp::IdeWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow;
}

impl IdeWindow {
    pub fn new(app: &gtk::Application, project: &Project) -> Self {
        debug!("Starting IdeWindow");

        let window: Self = Object::new(&[("application", app)]).expect("Failed to create WelcomeWindow");

        window.set_title(Some(project.path.to_string_lossy().to_string().as_str()));
        window.set_default_size(900, 700);

        return window;
    }
}
