use std::cell::RefCell;

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::NavigationSidebarItem)]
    #[template(file = "navigation_sidebar_item.ui")]
    pub struct NavigationSidebarItem {
        #[property(get, set, nullable)]
        icon_name: RefCell<Option<String>>,
        #[property(get, set, nullable)]
        label: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NavigationSidebarItem {
        const NAME: &'static str = "RpyNavigationSidebarItem";

        type Type = super::NavigationSidebarItem;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for NavigationSidebarItem {}
    impl WidgetImpl for NavigationSidebarItem {}
    impl ListBoxRowImpl for NavigationSidebarItem {}
}

glib::wrapper! {
    pub struct NavigationSidebarItem(ObjectSubclass<imp::NavigationSidebarItem>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl NavigationSidebarItem {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
