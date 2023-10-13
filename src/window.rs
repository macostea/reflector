use gtk::{
  glib,
  gio,
  CompositeTemplate, 
  subclass::prelude::*,
  prelude::*,
};

mod imp {
  use super::*;

  #[derive(Debug, Default, CompositeTemplate)]
  #[template(file = "../data/gtk/window.ui")]
  pub struct RflWindow {
    
  }

  #[glib::object_subclass]
  impl ObjectSubclass for RflWindow {
    const NAME: &'static str = "RflWindow";
    type Type = super::RflWindow;
    type ParentType = gtk::ApplicationWindow;

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
      obj.init_template();
    }

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        klass.bind_template_instance_callbacks();
    }
  }

  impl ObjectImpl for RflWindow {}
  impl WidgetImpl for RflWindow {}
  impl WindowImpl for RflWindow {}
  impl ApplicationWindowImpl for RflWindow {}
}

glib::wrapper! {
  pub struct RflWindow(ObjectSubclass<imp::RflWindow>)
    @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks]
impl RflWindow {
  pub fn new<A: IsA<gtk::Application>>(app: &A) -> Self {
    glib::Object::builder()
      .property("application", app)
      .build()
  }
}
