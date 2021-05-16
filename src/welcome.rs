// use crate::application::ExampleApplication;
// use crate::config::{APP_ID, PROFILE};
use glib::signal::Inhibit;
use gtk::subclass::prelude::*;
use gtk::{self, prelude::*};
use gtk::{gio, glib, CompositeTemplate};
use log::warn;

mod imp {
    use super::*;

    #[derive(Debug)]
    // #[template(resource = "/com/belmoussaoui/GtkRustTemplate/ui/window.ui")]
    pub struct ExampleApplicationWindow {
        // #[template_child]
        // pub headerbar: TemplateChild<gtk::HeaderBar>,
        // pub settings: gio::Settings,
    }

    // #[glib::object_subclass]
    impl ObjectSubclass for ExampleApplicationWindow {
        const NAME: &'static str = "ExampleApplicationWindow";
        type Type = super::ExampleApplicationWindow;
        type ParentType = gtk::ApplicationWindow;
        type Interfaces = 
        type Instance = 
        type Class =
        
        fn fn type_data() -> ptr::NonNull<TypeData> {
            
        }

        fn get_type() -> Type {
            
        }

        // fn new() -> Self {
        //     Self {
        //         headerbar: TemplateChild::default(),
        //         settings: gio::Settings::new(APP_ID),
        //     }
        // }

        // fn class_init(klass: &mut Self::Class) {
        //     Self::bind_template(klass);
        // }

        // // You must call `Widget`'s `init_template()` within `instance_init()`.
        // fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        //     obj.init_template();
        // }
    }

    impl ObjectImpl for ExampleApplicationWindow {}
    impl WidgetImpl for ExampleApplicationWindow {}
    impl WindowImpl for ExampleApplicationWindow {}

    impl ApplicationWindowImpl for ExampleApplicationWindow {}
}

glib::wrapper! {
    pub struct ExampleApplicationWindow(ObjectSubclass<imp::ExampleApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow;
}

impl ExampleApplicationWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let window: Self = glib::Object::new(&[]).expect("Failed to create ExampleApplicationWindow");
        window.set_application(Some(app));

        // Set icons for shell
        // gtk::Window::set_default_icon_name(APP_ID);

        return window;
    }
}
