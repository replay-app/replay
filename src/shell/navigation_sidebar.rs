use std::cell::RefCell;

use crate::prelude::*;
use crate::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::NavigationSidebar)]
    #[template(file = "navigation_sidebar.ui")]
    pub struct NavigationSidebar {
        #[template_child]
        list_box: TemplateChild<gtk::ListBox>,

        #[property(get, set = Self::set_pages, nullable)]
        pages: RefCell<Option<gio::ListModel>>,
        #[property(get, set, nullable)]
        selected_page: RefCell<Option<rpy::PageModel>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NavigationSidebar {
        const NAME: &'static str = "RpyNavigationSidebar";

        type Type = super::NavigationSidebar;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
            klass.set_css_name("rpy-navigation-sidebar");
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for NavigationSidebar {
        fn dispose(&self) {
            self.list_box.unparent();
        }
    }

    impl WidgetImpl for NavigationSidebar {}

    #[gtk::template_callbacks]
    impl NavigationSidebar {
        fn set_pages(&self, pages: Option<&gio::ListModel>) {
            if self.pages.borrow().is_some() {
                self.list_box.unset_header_func();
                self.list_box.unbind_model();
            }

            *self.pages.borrow_mut() = pages.cloned();

            if let Some(pages) = self.pages.borrow().as_ref() {
                self.list_box.bind_model(Some(pages), |page| {
                    let page = page
                        .downcast_ref()
                        .expect("'page' should be a 'rpy::PageModel'");

                    Self::create_row(page)
                });

                self.list_box.set_header_func(glib::clone!(
                    #[weak(rename_to = this)]
                    self,
                    move |row, before| this.create_header(row, before)
                ));
            }
        }

        fn create_row(page: &rpy::PageModel) -> gtk::Widget {
            let item = rpy::NavigationSidebarItem::new();

            page.bind_property("icon-name", &item, "icon-name")
                .sync_create()
                .build();

            page.bind_property("title", &item, "label")
                .sync_create()
                .build();

            item.upcast()
        }

        fn create_header(&self, row: &gtk::ListBoxRow, before: Option<&gtk::ListBoxRow>) {
            let obj = self.obj();

            let Some(before) = before else {
                return;
            };

            let pages = obj.pages().expect("'pages' should be set at this point");

            let previous_page = pages
                .item(before.index() as u32)
                .and_downcast::<rpy::PageModel>()
                .expect("'before' should match with a 'rpy::PageModel'");

            let current_page = pages
                .item(row.index() as u32)
                .and_downcast::<rpy::PageModel>()
                .expect("'row' should match with a 'rpy::PageModel'");

            if previous_page.page_type() == rpy::PageType::Main
                && current_page.page_type() == rpy::PageType::Library
            {
                row.set_header(Some(&{
                    let header = rpy::NavigationSidebarHeader::new();
                    // Translators: Title for the "Library" section.
                    header.set_label(Some(i18n::gettext("Library")));

                    header
                }));
            }
        }

        #[template_callback]
        fn on_row_selected(&self, row: Option<&gtk::ListBoxRow>) {
            let obj = self.obj();

            let Some(row) = row else {
                obj.set_selected_page(None::<rpy::PageModel>);
                return;
            };

            let pages = obj.pages().expect("'pages' should be set at this point");

            let page = pages
                .item(row.index() as u32)
                .and_downcast::<rpy::PageModel>()
                .expect("'row' should match with a 'rpy::PageModel'");

            obj.set_selected_page(Some(page));
        }
    }
}

glib::wrapper! {
    pub struct NavigationSidebar(ObjectSubclass<imp::NavigationSidebar>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl NavigationSidebar {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
