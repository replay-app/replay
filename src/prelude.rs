pub use adw::prelude::*;

pub use crate::shared::{constants::*, i18n, models::RpyPageModelExt};

pub mod rpy {
    pub use crate::app::*;
    pub use crate::shared::{enums::*, models::PageModel};
    pub use crate::shell::*;
}
