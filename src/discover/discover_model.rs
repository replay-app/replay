use std::cell::{Cell, RefCell};

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, glib::Properties)]
    #[properties(wrapper_type = super::DiscoverModel)]
    pub struct DiscoverModel {
        #[property(get, override_interface = rpy::PageModel)]
        icon_name: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        title: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        page_type: Cell<rpy::PageType>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DiscoverModel {
        const NAME: &'static str = "RpyDiscoverModel";

        type Type = super::DiscoverModel;
        type ParentType = glib::Object;
        type Interfaces = (rpy::PageModel,);

        fn new() -> Self {
            Self {
                icon_name: RefCell::new(Some("rpy-discover-symbolic".into())),
                // Translators: Title for the "Discover" page.
                title: RefCell::new(Some(i18n::gettext("Discover"))),
                page_type: Cell::new(rpy::PageType::Main),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for DiscoverModel {}
    impl RpyPageModelImpl for DiscoverModel {}
}

glib::wrapper! {
    pub struct DiscoverModel(ObjectSubclass<imp::DiscoverModel>)
        @implements rpy::PageModel;
}

impl DiscoverModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
