use application::RflApplication;
use gtk::{glib, prelude::*};


mod application;
mod window;
mod config;

static GRESOURCE_BYTES: &[u8] =
    gvdb_macros::include_gresource_from_dir!("/com/mcostea/Reflector", "data/resources");

fn main() -> glib::ExitCode {
    RflApplication::new().run()
}
