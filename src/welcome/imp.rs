use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct WelcomeWindow;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for WelcomeWindow {
    const NAME: &'static str = "WelcomeWindow";
    type Type = super::WelcomeWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for WelcomeWindow {}
impl WindowImpl for WelcomeWindow {}
impl WidgetImpl for WelcomeWindow {}
impl ApplicationWindowImpl for WelcomeWindow {}
