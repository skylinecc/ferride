use gtk::prelude::*;
use gtk::glib::clone;
use crate::project::Project;

pub struct MainWindow {
    window: gtk::ApplicationWindow,
    project: Project,
}

impl MainWindow {
    pub fn run(project: Project, app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_default_size(1200, 650);
        window.set_title(Some(project.name.as_str()));

        let myself = Self {
            window,
            project,
        };
        
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        h_box.set_valign(gtk::Align::Center);
        h_box.set_halign(gtk::Align::Center);
        h_box.set_vexpand(true);
        h_box.set_hexpand(true);
        
        let right_label = gtk::Label::new(Some("Right Panel"));
        right_label.set_halign(gtk::Align::Center);
        right_label.set_valign(gtk::Align::Center);
        h_box.append(&right_label);

        let left_label = gtk::Label::new(Some("Left Panel"));
        left_label.set_halign(gtk::Align::Center);
        left_label.set_valign(gtk::Align::Center);

        let revealer = gtk::Revealer::new();
        revealer.set_transition_type(gtk::RevealerTransitionType::SlideLeft);
        revealer.set_child(Some(&left_label));
        revealer.set_reveal_child(true);
        h_box.prepend(&revealer);

        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        v_box.append(&h_box);

        let button = gtk::Button::with_label("Open Revealer");
        
        println!("is_child_revealed({})", revealer.get_child_revealed());

        button.connect_clicked(clone!(@strong revealer => move |_| {
            println!("is_child_revealed({})", revealer.get_child_revealed());
            if revealer.get_child_revealed() {
                revealer.set_reveal_child(false);
            } else {
                revealer.set_reveal_child(true);
            };
        }));
        
        v_box.append(&button);

        myself.window.set_child(Some(&v_box));
        
        myself.window.present();

        return myself;
    }
}
