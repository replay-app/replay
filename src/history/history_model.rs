use std::cell::{Cell, RefCell};

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::HistoryModel)]
    pub struct HistoryModel {
        #[property(get, override_interface = rpy::PageModel)]
        icon_name: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        title: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        page_type: Cell<rpy::PageType>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HistoryModel {
        const NAME: &'static str = "RpyHistoryModel";

        type Type = super::HistoryModel;
        type ParentType = glib::Object;
        type Interfaces = (rpy::PageModel,);

        fn new() -> Self {
            Self {
                icon_name: RefCell::new(Some("rpy-history-symbolic".into())),
                // Translators: Title for the "History" page.
                title: RefCell::new(Some(i18n::gettext("History"))),
                page_type: Cell::new(rpy::PageType::Library),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for HistoryModel {}
    impl RpyPageModelImpl for HistoryModel {}
}

glib::wrapper! {
    pub struct HistoryModel(ObjectSubclass<imp::HistoryModel>)
        @implements rpy::PageModel;
}

impl HistoryModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
