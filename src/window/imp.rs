use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct IdeWindow;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for IdeWindow {
    const NAME: &'static str = "IdeWindow";
    type Type = super::IdeWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for IdeWindow {}
impl WindowImpl for IdeWindow {}
impl WidgetImpl for IdeWindow {}
impl ApplicationWindowImpl for IdeWindow {}
