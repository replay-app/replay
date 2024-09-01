use std::cell::{Cell, RefCell};

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, glib::Properties)]
    #[properties(wrapper_type = super::LikedVideosModel)]
    pub struct LikedVideosModel {
        #[property(get, override_interface = rpy::PageModel)]
        icon_name: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        title: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        page_type: Cell<rpy::PageType>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LikedVideosModel {
        const NAME: &'static str = "RpyLikedVideosModel";

        type Type = super::LikedVideosModel;
        type ParentType = glib::Object;
        type Interfaces = (rpy::PageModel,);

        fn new() -> Self {
            Self {
                icon_name: RefCell::new(Some("rpy-liked-videos-symbolic".into())),
                // Translators: Title for the "Liked Videos" page.
                title: RefCell::new(Some(i18n::gettext("Liked Videos"))),
                page_type: Cell::new(rpy::PageType::Library),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for LikedVideosModel {}
    impl RpyPageModelImpl for LikedVideosModel {}
}

glib::wrapper! {
    pub struct LikedVideosModel(ObjectSubclass<imp::LikedVideosModel>)
        @implements rpy::PageModel;
}

impl LikedVideosModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
