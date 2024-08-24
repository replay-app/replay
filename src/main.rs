use crate::prelude::*;

mod app;
mod prelude;
mod shared;
mod shell;
mod subclass;

fn main() -> anyhow::Result<glib::ExitCode> {
    i18n::init()?;
    Ok(rpy::App::new().run())
}
