use gtk::glib::{self, ParamSpec, Properties};
use adw::{prelude::*, subclass::prelude::*};
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::Pod)]
    pub struct Pod {
        #[property(name = "pod-name", get, set, type = String)]
        pub pod_name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Pod {
        const NAME: &'static str = "Pod";
        type Type = super::Pod;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for Pod {
        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec);
        }
    }
}

glib::wrapper! {
  pub struct Pod(ObjectSubclass<imp::Pod>);
}

impl Pod {
    pub fn new(name: String) -> Self {
        glib::Object::builder().property("pod-name", name).build()
    }
}
