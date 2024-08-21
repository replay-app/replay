use std::fmt::Display;

use gettext::{bind_textdomain_codeset, bindtextdomain, textdomain};
#[allow(unused_imports)]
pub use gettext::{gettext, ngettext, npgettext, pgettext};

use crate::prelude::*;

pub fn init() -> anyhow::Result<()> {
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR)?;
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")?;
    textdomain(GETTEXT_PACKAGE)?;

    Ok(())
}

// Taken from https://gitlab.gnome.org/World/warp/-/blob/8072954d2ab9bd875fdf66860f44e5d4913b5d3e/src/gettext.rs#L6-L16
fn fmt(mut format: String, args: &[&dyn Display]) -> String {
    for arg in args {
        format = format.replacen("{}", &arg.to_string(), 1);
    }

    for (i, arg) in args.iter().enumerate() {
        format = format.replace(&format!("{{{i}}}"), &arg.to_string());
    }

    format
}

#[allow(dead_code)]
pub fn gettextf<T: Into<String>>(msgid: T, args: &[&dyn Display]) -> String {
    fmt(gettext(msgid), args)
}

#[allow(dead_code)]
pub fn pgettextf<T: Into<String>, U: Into<String>>(
    msgctxt: T,
    msgid: U,
    args: &[&dyn Display],
) -> String {
    fmt(pgettext(msgctxt, msgid), args)
}

#[allow(dead_code)]
pub fn ngettextf<T: Into<String>, S: Into<String>>(
    msgid: T,
    msgid_plural: S,
    n: u32,
    args: &[&dyn Display],
) -> String {
    fmt(ngettext(msgid, msgid_plural, n), args)
}

#[allow(dead_code)]
pub fn npgettextf<T: Into<String>, U: Into<String>, V: Into<String>>(
    msgctxt: T,
    msgid: U,
    msgid_plural: V,
    n: u32,
    args: &[&dyn Display],
) -> String {
    fmt(npgettext(msgctxt, msgid, msgid_plural, n), args)
}
