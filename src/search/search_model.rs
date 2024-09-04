use std::cell::{Cell, RefCell};

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::SearchModel)]
    pub struct SearchModel {
        #[property(get, override_interface = rpy::PageModel)]
        icon_name: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        title: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        page_type: Cell<rpy::PageType>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SearchModel {
        const NAME: &'static str = "RpySearchModel";

        type Type = super::SearchModel;
        type ParentType = glib::Object;
        type Interfaces = (rpy::PageModel,);

        fn new() -> Self {
            Self {
                icon_name: RefCell::new(Some("rpy-search-symbolic".into())),
                // Translators: Title for the "Search" page.
                title: RefCell::new(Some(i18n::gettext("Search"))),
                page_type: Cell::new(rpy::PageType::Search),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for SearchModel {}
    impl RpyPageModelImpl for SearchModel {}
}

glib::wrapper! {
    pub struct SearchModel(ObjectSubclass<imp::SearchModel>)
        @implements rpy::PageModel;
}

impl SearchModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
