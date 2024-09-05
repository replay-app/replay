use std::cell::RefCell;

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::ShellDesktopLayout)]
    #[template(file = "shell_desktop_layout.ui")]
    pub struct ShellDesktopLayout {
        #[template_child]
        split_view: TemplateChild<adw::OverlaySplitView>,
        #[template_child]
        nav_sidebar_filter: TemplateChild<gtk::CustomFilter>,

        #[property(get, set, nullable)]
        model: RefCell<Option<rpy::ShellModel>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ShellDesktopLayout {
        const NAME: &'static str = "RpyShellDesktopLayout";

        type Type = super::ShellDesktopLayout;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for ShellDesktopLayout {
        fn constructed(&self) {
            self.parent_constructed();

            self.nav_sidebar_filter.set_filter_func(|page| {
                let page = page
                    .downcast_ref::<rpy::PageModel>()
                    .expect("'page' should be a 'rpy::PageModel'");

                let page_type = page.page_type();

                page_type != rpy::PageType::Search
            });
        }

        fn dispose(&self) {
            self.split_view.unparent();
        }
    }

    impl WidgetImpl for ShellDesktopLayout {}
}

glib::wrapper! {
    pub struct ShellDesktopLayout(ObjectSubclass<imp::ShellDesktopLayout>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ShellDesktopLayout {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
