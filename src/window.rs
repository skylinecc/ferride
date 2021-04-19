use gtk::prelude::*;
use crate::project::Project;

pub struct MainWindow {
    window: gtk::ApplicationWindow,
    project: Project,
}

impl MainWindow {
    pub fn run(project: Project, app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_default_size(1200, 650);

        let myself = Self {
            window,
            project,
        };
        
        myself.window.set_title(Some(myself.project.name.as_str()));

        let window_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        myself.window.set_child(Some(&window_box));
        window_box.set_margin_end(12);
        window_box.set_margin_start(12);
        window_box.set_margin_top(12);
        window_box.set_margin_bottom(12);

        let right_label = gtk::Label::new(Some("Right Panel"));
        let left_label = gtk::Label::new(Some("Left Panel"));

        let properties_panel = gtk::Revealer::new();
        properties_panel.set_transition_type(gtk::RevealerTransitionType::SwingRight);
        let properties_label = gtk::Label::new(Some("Properties Panel"));
        properties_label.set_focus_child(Some(&properties_label));

        window_box.append(&properties_panel);
        properties_panel.set_reveal_child(true);
        
        myself.window.present();

        return myself;
    }
}
