use crate::prelude::*;

mod prelude;
mod shared;

fn main() -> anyhow::Result<()> {
    i18n::init()?;

    println!("{}", i18n::gettext("Hello, world!"));

    Ok(())
}
