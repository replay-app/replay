use std::cell::{Cell, RefCell};

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, glib::Properties)]
    #[properties(wrapper_type = super::SubscriptionsModel)]
    pub struct SubscriptionsModel {
        #[property(get, override_interface = rpy::PageModel)]
        icon_name: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        title: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        page_type: Cell<rpy::PageType>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SubscriptionsModel {
        const NAME: &'static str = "RpySubscriptionsModel";

        type Type = super::SubscriptionsModel;
        type ParentType = glib::Object;
        type Interfaces = (rpy::PageModel,);

        fn new() -> Self {
            Self {
                icon_name: RefCell::new(Some("rpy-subscriptions-symbolic".into())),
                // Translators: Title for the "Subscriptions" page.
                title: RefCell::new(Some(i18n::gettext("Subscriptions"))),
                page_type: Cell::new(rpy::PageType::Main),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for SubscriptionsModel {}
    impl RpyPageModelImpl for SubscriptionsModel {}
}

glib::wrapper! {
    pub struct SubscriptionsModel(ObjectSubclass<imp::SubscriptionsModel>)
        @implements rpy::PageModel;
}

impl SubscriptionsModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
