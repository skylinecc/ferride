use gtk::prelude::*;
use std::path::PathBuf;
use crate::project::get_name;

pub struct MainWindow {
    window: gtk::ApplicationWindow,
}

impl MainWindow {
    pub fn run(path: PathBuf, app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_default_size(1200, 650);

        let package_name = match get_name(path) {
            Ok(data) => data,
            Err(error) => {
                eprintln!("error getting package name: {}", error);
                std::process::exit(1);
            }
        };

        window.set_title(Some(&format!("{} - ride", package_name)));

        window.present();

        Self {
            window,
        }
    }
}
