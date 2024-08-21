use default_env::default_env;

pub const GETTEXT_PACKAGE: &'static str = default_env!("GETTEXT_PACKAGE", "replay");
pub const LOCALEDIR: &'static str = default_env!("LOCALEDIR", "/app/share/locale");
