use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "shell_window.ui")]
    pub struct ShellWindow {
        #[template_child]
        navigation_view: TemplateChild<adw::NavigationView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ShellWindow {
        const NAME: &'static str = "RpyShellWindow";

        type Type = super::ShellWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ShellWindow {}
    impl WidgetImpl for ShellWindow {}
    impl WindowImpl for ShellWindow {}
    impl ApplicationWindowImpl for ShellWindow {}
    impl AdwApplicationWindowImpl for ShellWindow {}
}

glib::wrapper! {
    pub struct ShellWindow(ObjectSubclass<imp::ShellWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl ShellWindow {
    pub fn new(app: &impl IsA<gtk::Application>) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
