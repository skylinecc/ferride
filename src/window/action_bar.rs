use gtk::prelude::*;
use gtk::glib::clone;
use crate::window::ApplicationWindow;

pub struct ActionBar {
    pub bar: gtk::ActionBar,
}

impl ActionBar {
    pub fn new(window: &ApplicationWindow) -> Self {
        let bar = gtk::ActionBar::new();

        let run_button = gtk::Button::from_icon_name(Some("system-run"));
        let left_panel_button = gtk::ToggleButton::new();

        let orientation_portrait_left = gtk::Image::from_resource("/org/skylinecc/Ferride/images/dock-left-symbolic.svg");
        left_panel_button.set_child(Some(&orientation_portrait_left));
        left_panel_button.set_active(true);
        left_panel_button.set_action_name(Some("app.toggle_side_panel"));

        bar.pack_start(&left_panel_button);
        bar.pack_end(&run_button);

        return Self { bar };
    }
}