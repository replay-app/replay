use default_env::default_env;

pub const APP_ID: &'static str = default_env!("APP_ID", "one.naiara.Replay");
pub const GETTEXT_PACKAGE: &'static str = default_env!("GETTEXT_PACKAGE", "replay");
pub const LOCALEDIR: &'static str = default_env!("LOCALEDIR", "/app/share/locale");
