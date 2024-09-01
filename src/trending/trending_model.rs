use std::cell::{Cell, RefCell};

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, glib::Properties)]
    #[properties(wrapper_type = super::TrendingModel)]
    pub struct TrendingModel {
        #[property(get, override_interface = rpy::PageModel)]
        icon_name: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        title: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        page_type: Cell<rpy::PageType>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TrendingModel {
        const NAME: &'static str = "RpyTrendingModel";

        type Type = super::TrendingModel;
        type ParentType = glib::Object;
        type Interfaces = (rpy::PageModel,);

        fn new() -> Self {
            Self {
                icon_name: RefCell::new(Some("rpy-trending-symbolic".into())),
                // Translators: Title for the "Trending" page.
                title: RefCell::new(Some(i18n::gettext("Trending"))),
                page_type: Cell::new(rpy::PageType::Main),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for TrendingModel {}
    impl RpyPageModelImpl for TrendingModel {}
}

glib::wrapper! {
    pub struct TrendingModel(ObjectSubclass<imp::TrendingModel>)
        @implements rpy::PageModel;
}

impl TrendingModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
