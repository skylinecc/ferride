use gtk::prelude::*;
use gtk::gio;
use gio::prelude::*;

pub struct MenuBar {
    pub model: gio::Menu,
    pub bar: gtk::PopoverMenuBar,
}

impl MenuBar {
    pub fn new() -> Self {
        let model = gio::Menu::new();
        model.append(Some("File"), None);

        let bar = gtk::PopoverMenuBar::from_model(Some(&model));

        Self {
            model,
            bar,
        }
    }
}