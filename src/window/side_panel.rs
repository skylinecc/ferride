use gtk::prelude::*;

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct SidePanel {
    pub revealer: gtk::Revealer,
    pub side_panel_box: gtk::Box,
}

impl SidePanel {
    pub fn new() -> Self {
        let revealer = gtk::Revealer::new();
        let side_panel_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let temp_label = gtk::Label::new(Some("Side Panel"));
        temp_label.set_halign(gtk::Align::Center);
        temp_label.set_valign(gtk::Align::Center);
        temp_label.set_hexpand(true);
        temp_label.set_vexpand(true);
        side_panel_box.append(&temp_label);

        let vertical_separator = gtk::Separator::new(gtk::Orientation::Vertical);
        side_panel_box.append(&vertical_separator);

        revealer.set_transition_type(gtk::RevealerTransitionType::SlideRight);
        revealer.set_child(Some(&side_panel_box));
        revealer.set_reveal_child(true);
    
        let side_panel = Self {
            revealer,
            side_panel_box,
        };

        return side_panel;
    }
}