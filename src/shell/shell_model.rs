use std::cell::RefCell;

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, glib::Properties)]
    #[properties(wrapper_type = super::ShellModel)]
    pub struct ShellModel {
        #[property(get)]
        pages: RefCell<gio::ListModel>,
        #[property(get, set)]
        current_page: RefCell<Option<rpy::PageModel>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ShellModel {
        const NAME: &'static str = "RpyShellModel";

        type Type = super::ShellModel;
        type ParentType = glib::Object;

        fn new() -> Self {
            Self {
                pages: RefCell::new(gio::ListStore::new::<rpy::PageModel>().upcast()),
                current_page: RefCell::new(None),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for ShellModel {
        fn constructed(&self) {
            self.parent_constructed();

            let pages = self.pages.borrow();
            let pages = pages
                .downcast_ref::<gio::ListStore>()
                .expect("'pages' should be a 'gio::ListStore'");

            let additions: &[rpy::PageModel] = &[
                rpy::DiscoverModel::new().upcast(),
                rpy::TrendingModel::new().upcast(),
                rpy::SubscriptionsModel::new().upcast(),
                rpy::LikedVideosModel::new().upcast(),
                rpy::FavoritesModel::new().upcast(),
                rpy::HistoryModel::new().upcast(),
                rpy::WatchLaterModel::new().upcast(),
            ];

            pages.splice(0, 0, additions);
        }
    }
}

glib::wrapper! {
    pub struct ShellModel(ObjectSubclass<imp::ShellModel>);
}

impl ShellModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
