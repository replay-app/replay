use std::cell::RefCell;

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::NavigationSidebarHeader)]
    #[template(file = "navigation_sidebar_header.ui")]
    pub struct NavigationSidebarHeader {
        #[template_child]
        label_widget: TemplateChild<gtk::Label>,

        #[property(get, set, nullable)]
        label: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NavigationSidebarHeader {
        const NAME: &'static str = "RpyNavigationSidebarHeader";

        type Type = super::NavigationSidebarHeader;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.set_css_name("rpy-navigation-sidebar-header");
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for NavigationSidebarHeader {
        fn dispose(&self) {
            self.label_widget.unparent();
        }
    }

    impl WidgetImpl for NavigationSidebarHeader {}
}

glib::wrapper! {
    pub struct NavigationSidebarHeader(ObjectSubclass<imp::NavigationSidebarHeader>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl NavigationSidebarHeader {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
