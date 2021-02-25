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

        let window_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        myself.window.set_child(Some(&window_box));
        window_box.set_margin_end(12);
        window_box.set_margin_start(12);
        window_box.set_margin_top(12);
        window_box.set_margin_bottom(12);

        // Action Workspace
        let action_frame = gtk::Frame::new(Some("Action Frame"));
        window_box.append(&action_frame);
        let action_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        action_box.set_halign(gtk::Align::Center);
        action_frame.set_child(Some(&action_box));

        // Content Pane (left vs right)
        let content_h_pane = gtk::Paned::new(gtk::Orientation::Horizontal);
        window_box.append(&content_h_pane);
        content_h_pane.set_vexpand(true);

        // File Manager
        let file_manager_frame = gtk::Frame::new(Some("File Manager"));
        content_h_pane.set_start_child(&file_manager_frame);
        let file_manager_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        file_manager_frame.set_child(Some(&file_manager_box));

        // Content Pane (top vs bottom)
        let content_v_pane = gtk::Paned::new(gtk::Orientation::Vertical);
        content_h_pane.set_end_child(&content_v_pane);

        // Text Editor
        let text_editor_frame = gtk::Frame::new(Some("Text Editor"));
        content_v_pane.set_start_child(&text_editor_frame);
        let text_editor_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        text_editor_frame.set_child(Some(&text_editor_box));

        // Terminal
        let terminal_frame = gtk::Frame::new(Some("Terminal"));
        content_v_pane.set_end_child(&terminal_frame);
        let terminal_box = gtk::Box::new(gtk::Orientation:: Vertical, 0);
        terminal_frame.set_child(Some(&terminal_box));

        myself.set_project_details(path);
        myself.window.present();

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
