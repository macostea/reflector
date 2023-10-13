use application::RflApplication;
use gtk::{glib, prelude::*};
use once_cell::sync::Lazy;

mod application;
mod config;
mod models;
mod pod_row;
mod utils;
mod window;

// static GRESOURCE_BYTES: &[u8] =
//     gvdb_macros::include_gresource_from_dir!("/com/mcostea/Reflector", "data/resources");

pub static RUNTIME: Lazy<tokio::runtime::Runtime> =
    Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

pub static MAINT_CONTEXT: Lazy<glib::MainContext> = Lazy::new(|| glib::MainContext::default());

fn main() -> glib::ExitCode {
    RflApplication::new().run()
}
