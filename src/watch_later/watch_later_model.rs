use std::cell::{Cell, RefCell};

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, glib::Properties)]
    #[properties(wrapper_type = super::WatchLaterModel)]
    pub struct WatchLaterModel {
        #[property(get, override_interface = rpy::PageModel)]
        icon_name: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        title: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        page_type: Cell<rpy::PageType>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WatchLaterModel {
        const NAME: &'static str = "RpyWatchLaterModel";

        type Type = super::WatchLaterModel;
        type ParentType = glib::Object;
        type Interfaces = (rpy::PageModel,);

        fn new() -> Self {
            Self {
                icon_name: RefCell::new(Some("rpy-watch-later-symbolic".into())),
                // Translators: Title for the "Watch Later" page.
                title: RefCell::new(Some(i18n::gettext("Watch Later"))),
                page_type: Cell::new(rpy::PageType::Library),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for WatchLaterModel {}
    impl RpyPageModelImpl for WatchLaterModel {}
}

glib::wrapper! {
    pub struct WatchLaterModel(ObjectSubclass<imp::WatchLaterModel>)
        @implements rpy::PageModel;
}

impl WatchLaterModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
