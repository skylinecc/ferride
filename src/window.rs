use gtk::prelude::*;
use std::path::PathBuf;

pub struct MainWindow {
    window: gtk::ApplicationWindow,
}

impl MainWindow {
    pub fn run(path: PathBuf, app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);


        window.present();

        Self {
            window,
        }
    }
}
