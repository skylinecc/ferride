use gtk::prelude::*;
use gtk::glib::clone;
use crate::project::Project;

#[derive(Clone)]
pub struct MainWindow {
    application: gtk::Application,
    window: gtk::ApplicationWindow,
    project: Project,
    side_panel_revealer: gtk::Revealer,
}

impl MainWindow {
    pub fn run(project: Project, app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_default_size(1200, 650);
        window.set_title(Some(&format!("Ferride - {}", project.name.as_str())));
        // window.set_position(gtk::WindowPosition::Center);

        let application = app.clone();
        let side_panel_revealer = gtk::Revealer::new();

        let myself = Self {
            application,
            window,
            project,
            side_panel_revealer,
        };

        let content_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        myself.window.set_child(Some(&content_box));

        let action_bar = myself.action_bar();
        content_box.append(&action_bar);

        let action_bar_separator = gtk::Separator::new(gtk::Orientation::Horizontal);
        content_box.append(&action_bar_separator);
              
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        h_box.set_valign(gtk::Align::Center);
        h_box.set_halign(gtk::Align::Center);
        h_box.set_vexpand(true);
        h_box.set_hexpand(true);
        content_box.append(&h_box);
        
        let right_label = gtk::Label::new(Some("Right Panel"));
        right_label.set_halign(gtk::Align::Center);
        right_label.set_valign(gtk::Align::Center);
        right_label.set_hexpand(true);
        right_label.set_vexpand(true);
        h_box.append(&right_label);

        let left_label = gtk::Label::new(Some("Left Panel"));
        left_label.set_halign(gtk::Align::Center);
        left_label.set_valign(gtk::Align::Center);
        left_label.set_hexpand(true);
        left_label.set_vexpand(true);

        myself.side_panel_revealer.set_transition_type(gtk::RevealerTransitionType::SlideLeft);
        myself.side_panel_revealer.set_child(Some(&left_label));
        myself.side_panel_revealer.set_reveal_child(true);
        h_box.prepend(&myself.side_panel_revealer);

        myself.window.present();

        return myself;
    }

    fn action_bar(&self) -> gtk::ActionBar {
        let bar = gtk::ActionBar::new();

        let run_button = gtk::Button::from_icon_name(Some("system-run"));
        let left_panel_button = gtk::ToggleButton::new();

        let orientation_portrait_left = gtk::Image::from_resource("/org/skylinecc/Ferride/images/dock-left-symbolic.svg");
        left_panel_button.set_child(Some(&orientation_portrait_left));
        left_panel_button.set_active(true);
        
        left_panel_button.connect_toggled(clone!(@strong self as myself => move |_| {
            myself.toggle_side_panel_revealed();
        }));

        bar.pack_start(&left_panel_button);
        bar.pack_end(&run_button);

        return bar;
    }

    fn toggle_side_panel_revealed(&self) {
        if self.side_panel_revealer.get_child_revealed() {
            self.side_panel_revealer.set_reveal_child(false);
        } else {
            self.side_panel_revealer.set_reveal_child(true);
        }
    }
}
