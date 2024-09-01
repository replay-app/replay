use std::cell::{Cell, RefCell};

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, glib::Properties)]
    #[properties(wrapper_type = super::FavoritesModel)]
    pub struct FavoritesModel {
        #[property(get, override_interface = rpy::PageModel)]
        icon_name: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        title: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        page_type: Cell<rpy::PageType>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FavoritesModel {
        const NAME: &'static str = "RpyFavoritesModel";

        type Type = super::FavoritesModel;
        type ParentType = glib::Object;
        type Interfaces = (rpy::PageModel,);

        fn new() -> Self {
            Self {
                icon_name: RefCell::new(Some("rpy-favorites-symbolic".into())),
                // Translators: Title for the "Favorites" page.
                title: RefCell::new(Some(i18n::gettext("Favorites"))),
                page_type: Cell::new(rpy::PageType::Library),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for FavoritesModel {}
    impl RpyPageModelImpl for FavoritesModel {}
}

glib::wrapper! {
    pub struct FavoritesModel(ObjectSubclass<imp::FavoritesModel>)
        @implements rpy::PageModel;
}

impl FavoritesModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
