#[derive(Clone, Copy, Debug, Default, Eq, glib::Enum, PartialEq)]
#[enum_type(name = "RpyPageType")]
pub enum PageType {
    #[default]
    Main,
    Library,
    Search,
}
