use std::cell::{Cell, RefCell};

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::PlaylistsModel)]
    pub struct PlaylistsModel {
        #[property(get, override_interface = rpy::PageModel)]
        icon_name: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        title: RefCell<Option<String>>,
        #[property(get, override_interface = rpy::PageModel)]
        page_type: Cell<rpy::PageType>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PlaylistsModel {
        const NAME: &'static str = "RpyPlaylistsModel";

        type Type = super::PlaylistsModel;
        type ParentType = glib::Object;
        type Interfaces = (rpy::PageModel,);

        fn new() -> Self {
            Self {
                icon_name: RefCell::new(Some("rpy-playlists-symbolic".into())),
                // Translators: Title for the "Playlists" page.
                title: RefCell::new(Some(i18n::gettext("Playlists"))),
                page_type: Cell::new(rpy::PageType::Library),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for PlaylistsModel {}
    impl RpyPageModelImpl for PlaylistsModel {}
}

glib::wrapper! {
    pub struct PlaylistsModel(ObjectSubclass<imp::PlaylistsModel>)
        @implements rpy::PageModel;
}

impl PlaylistsModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
