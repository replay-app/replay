pub use adw::prelude::*;

pub use crate::shared::{constants::*, i18n, models::RpyPageModelExt};

pub mod rpy {
    pub use crate::app::*;
    pub use crate::discover::*;
    pub use crate::favorites::*;
    pub use crate::liked_videos::*;
    pub use crate::shared::{enums::*, models::PageModel};
    pub use crate::shell::*;
    pub use crate::subscriptions::*;
    pub use crate::trending::*;
}
