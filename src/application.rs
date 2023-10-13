use crate::{config, window::RflWindow};
use gtk::{gio, glib, prelude::*, subclass::prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct RflApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for RflApplication {
        const NAME: &'static str = "RflApplication";
        type Type = super::RflApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for RflApplication {}
    impl ApplicationImpl for RflApplication {
        fn activate(&self) {
            let application = self.obj();
            let window = RflWindow::new(&*application);
            window.present();
        }
    }

    impl GtkApplicationImpl for RflApplication {}
}

glib::wrapper! {
  pub struct RflApplication(ObjectSubclass<imp::RflApplication>)
    @extends gtk::Application, gio::Application,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl RflApplication {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", config::APP_ID)
            .property("resource-base-path", "/com/mcostea/Reflector")
            .build()
    }
}
