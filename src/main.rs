use gvdb_macros::include_gresource_from_dir;

use crate::prelude::*;

mod app;
mod discover;
mod prelude;
mod shared;
mod shell;
mod subclass;
mod trending;

fn main() -> anyhow::Result<glib::ExitCode> {
    i18n::init()?;

    static GRESOURCE_BYTES: &[u8] = include_gresource_from_dir!("/one/naiara/Replay", "res");
    gio::resources_register(&gio::Resource::from_data(&glib::Bytes::from_static(
        GRESOURCE_BYTES,
    ))?);

    Ok(rpy::App::new().run())
}
