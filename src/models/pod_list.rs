use crate::models::pod::Pod;
use gtk::{gio::ListModel, glib, prelude::*, subclass::prelude::*};
use indexmap::IndexMap;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PodList {
        pub list: RefCell<IndexMap<usize, Pod>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PodList {
        const NAME: &'static str = "PodList";
        type Type = super::PodList;
        type Interfaces = (ListModel,);
    }

    impl ObjectImpl for PodList {}

    impl ListModelImpl for PodList {
        fn item_type(&self) -> glib::Type {
            Pod::static_type()
        }

        fn n_items(&self) -> u32 {
            self.list.borrow().len() as u32
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            self.list
                .borrow()
                .get_index(position as usize)
                .map(|(_, v)| v.upcast_ref::<glib::Object>())
                .cloned()
        }
    }
}

glib::wrapper! {
  pub struct PodList(ObjectSubclass<imp::PodList>)
    @implements ListModel;
}

impl PodList {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
