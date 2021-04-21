use gtk::prelude::*;
use gtk::glib::clone;
use gtk::gio::SimpleAction;
use crate::project::Project;

pub struct MainWindow {
    application: gtk::Application,
    window: gtk::ApplicationWindow,
    project: Project,
}

impl MainWindow {
    pub fn run(project: Project, app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_default_size(1200, 650);
        window.set_title(Some(project.name.as_str()));

        let application = app.clone();

        let myself = Self {
            application,
            window,
            project,
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
        h_box.append(&right_label);

        let left_label = gtk::Label::new(Some("Left Panel"));
        left_label.set_halign(gtk::Align::Center);
        left_label.set_valign(gtk::Align::Center);

        let side_panel_revealer = gtk::Revealer::new();
        side_panel_revealer.set_transition_type(gtk::RevealerTransitionType::SlideLeft);
        side_panel_revealer.set_child(Some(&left_label));
        side_panel_revealer.set_reveal_child(true);
        h_box.prepend(&side_panel_revealer);

        let side_panel_toggle_hidden = SimpleAction::new("ToggleSidePanel", None);
        side_panel_toggle_hidden.connect_activate(move |_, _| {
            println!("toggled!");
        });
        myself.application.set_accels_for_action("win.ToggleSidePanel", &["<Control>equal", "<Control>plus", "<Control>KP_Add"]);

        myself.window.present();

        return myself;
    }

    fn action_bar(&self) -> gtk::ActionBar {
        let bar = gtk::ActionBar::new();

        let run_button = gtk::Button::from_icon_name(Some("system-run"));
        let left_panel_button = gtk::Button::from_icon_name(Some("orientation-portrait-left"));
        left_panel_button.connect_clicked(move |_| {
            println!("Clicked hide left panel");
        });

        bar.pack_start(&left_panel_button);
        bar.pack_end(&run_button);

        return bar;
    }
}
