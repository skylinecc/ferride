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

        let mut myself = Self {
            window,
        };

        myself.window.present();
        myself.set_project_details(path);

        return myself;
    }

    fn set_project_details(&mut self, path: PathBuf) {
        let package_name = match get_name(path) {
            Ok(data) => data,
            Err(error) => {
                eprintln!("error getting package name: {}", error);
                std::process::exit(1);
            }
        };

        self.window.set_title(Some(&format!("{} - Ride", package_name)));
    }
}
