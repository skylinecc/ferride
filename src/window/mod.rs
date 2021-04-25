mod side_panel;
mod action_bar;

use side_panel::SidePanel;

use gtk::prelude::*;
use gtk::glib::clone;
use gtk::gio;
use crate::project::Project;

const SIDE_PANEL_SIZE_FACTOR: i32 = 4;

#[derive(Clone)]
pub struct ApplicationWindow {
    pub application: gtk::Application,
    pub window: gtk::ApplicationWindow,
    pub project: Project,
    pub side_panel: SidePanel,
    pub window_box: gtk::Box,
    pub content_box: gtk::Box,
    pub content_pane: gtk::Paned,
}

impl ApplicationWindow {
    pub fn run(project: Project, app: &gtk::Application) -> Self {
        let application = app.clone();
        let side_panel = SidePanel::new();
        let window_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let content_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let content_pane = gtk::Paned::new(gtk::Orientation::Horizontal);

        let window = gtk::ApplicationWindow::new(app);
        window.set_default_size(1200, 650);
        window.set_title(Some(&format!("{} - Ferride", project.name.as_str())));
        window.set_child(Some(&window_box));
        // window.set_position(gtk::WindowPosition::Center);

        let myself = Self {
            application,
            window,
            project,
            side_panel,
            window_box,
            content_box,
            content_pane,
        };
        
        myself.window.present();
        myself.actions();
        myself.build_ui();

        return myself;
    }
    
    fn build_ui(&self) {
        let action_bar = action_bar::ActionBar::new();

        self.window_box.append(&action_bar);
        self.window_box.append(&self.content_box);

        let right_label = gtk::Label::new(Some("Main Content"));
        right_label.set_hexpand(true);
        right_label.set_vexpand(true);

        self.content_pane.set_start_child(&self.side_panel.revealer);
        self.content_pane.set_wide_handle(false);
        self.content_pane.set_end_child(&right_label);

        let pane_position = self.window.get_size(gtk::Orientation::Horizontal) as i32 / SIDE_PANEL_SIZE_FACTOR;
        self.content_pane.set_position(pane_position);
        
        self.content_box.append(&self.content_pane);
    }

    pub fn actions(&self) {
        let toggle_side_panel = gio::SimpleAction::new("toggle_side_panel", None);
        toggle_side_panel.connect_activate(clone!(@strong self as myself => move |_, _| {
            if myself.side_panel.revealer.get_child_revealed() {
                trace!("Hiding side panel revealer");
                myself.content_pane.set_position(0);
                myself.side_panel.revealer.set_reveal_child(false);
            } else {
                trace!("Revealing side panel");
                let pane_position = myself.window.get_size(gtk::Orientation::Horizontal) as i32 / SIDE_PANEL_SIZE_FACTOR;

                myself.content_pane.set_position(pane_position);
                myself.side_panel.revealer.set_reveal_child(true);
            }
        }));
        self.application.add_action(&toggle_side_panel);
    }
}
