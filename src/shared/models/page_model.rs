use std::sync::OnceLock;

use crate::prelude::*;
use crate::subclass::prelude::*;

mod ffi {
    use super::*;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Interface {
        parent_iface: glib::gobject_ffi::GTypeInterface,
    }

    unsafe impl InterfaceStruct for Interface {
        type Type = super::iface::PageModel;
    }
}

mod iface {
    use super::*;

    pub struct PageModel;

    #[glib::object_interface]
    impl ObjectInterface for PageModel {
        const NAME: &'static str = "RpyPageModel";

        type Interface = super::ffi::Interface;
        type Prerequisites = (glib::Object,);

        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();

            PROPERTIES.get_or_init(|| {
                vec![
                    glib::ParamSpecString::builder("icon-name")
                        .read_only()
                        .build(),
                    glib::ParamSpecString::builder("title").read_only().build(),
                    glib::ParamSpecEnum::builder::<rpy::PageType>("page-type")
                        .read_only()
                        .build(),
                ]
            })
        }
    }
}

glib::wrapper! {
    pub struct PageModel(ObjectInterface<iface::PageModel>);
}

pub trait RpyPageModelExt: IsA<PageModel> {
    #[allow(dead_code)]
    fn icon_name(&self) -> Option<String> {
        self.property("icon-name")
    }

    #[allow(dead_code)]
    fn title(&self) -> Option<String> {
        self.property("title")
    }

    #[allow(dead_code)]
    fn page_type(&self) -> rpy::PageType {
        self.property("page-type")
    }
}

impl<T: IsA<PageModel>> RpyPageModelExt for T {}

pub trait RpyPageModelImpl: ObjectImpl {}

unsafe impl<Obj: RpyPageModelImpl> IsImplementable<Obj> for PageModel {}
