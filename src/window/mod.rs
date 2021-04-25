mod side_panel;
mod action_bar;
mod menu_bar;

use side_panel::SidePanel;

use gtk::prelude::*;
use gtk::glib::clone;
use gtk::gio;
use crate::project::Project;

#[derive(Clone)]
pub struct ApplicationWindow {
    pub application: gtk::Application,
    pub window: gtk::ApplicationWindow,
    pub project: Project,
    pub side_panel: SidePanel,
    pub window_box: gtk::Box,
    pub content_box: gtk::Box,
}

impl ApplicationWindow {
    pub fn run(project: Project, app: &gtk::Application) -> Self {
        let application = app.clone();
        let side_panel = SidePanel::new();
        let window_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let content_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

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
        };
        
        myself.actions();
        myself.build_ui();
        myself.window.present();

        return myself;
    }
    
    fn build_ui(&self) {
        let menu_bar = menu_bar::MenuBar::new();
        self.window_box.append(&menu_bar.bar);

        let action_bar = action_bar::ActionBar::new(&self);
        self.window_box.append(&action_bar.bar);

        self.window_box.append(&self.content_box);


        let revealer = gtk::Revealer::new();
        let content_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let temp_label = gtk::Label::new(Some("Side Panel"));
        temp_label.set_halign(gtk::Align::Center);
        temp_label.set_valign(gtk::Align::Center);
        temp_label.set_hexpand(true);
        temp_label.set_vexpand(true);
        content_box.append(&temp_label);

        let vertical_separator = gtk::Separator::new(gtk::Orientation::Vertical);
        content_box.append(&vertical_separator);

        revealer.set_transition_type(gtk::RevealerTransitionType::SlideLeft);
        revealer.set_child(Some(&content_box));
        revealer.set_reveal_child(true);
        self.content_box.append(&revealer);
    }

    pub fn actions(&self) {
        let toggle_side_panel = gio::SimpleAction::new("toggle_side_panel", None);
        toggle_side_panel.connect_activate(clone!(@strong self as myself => move |_, _| {
            if myself.side_panel.revealer.get_child_revealed() {
                trace!("Hiding side panel revealer");
                myself.side_panel.revealer.set_reveal_child(false);
            } else {
                trace!("Revealing side panel");
                myself.side_panel.revealer.set_reveal_child(true);
            }
        }));
        self.application.add_action(&toggle_side_panel);


    }
}
