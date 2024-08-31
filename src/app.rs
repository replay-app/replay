use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct App;

    #[glib::object_subclass]
    impl ObjectSubclass for App {
        const NAME: &'static str = "RpyApp";

        type Type = super::App;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for App {}

    impl ApplicationImpl for App {
        fn activate(&self) {
            let window = match self.obj().active_window() {
                Some(window) => window,
                None => rpy::ShellWindow::new(&*self.obj()).upcast(),
            };

            window.present();
        }

        fn startup(&self) {
            self.parent_startup();
            glib::set_application_name("Replay");
        }
    }

    impl GtkApplicationImpl for App {}
    impl AdwApplicationImpl for App {}
}

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::App>)
        @extends adw::Application, gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl App {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .property("resource-base-path", "/one/naiara/Replay")
            .build()
    }
}
