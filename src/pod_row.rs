use std::cell::RefCell;

use gtk::{
    glib::{self, Binding},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

use crate::models::pod::Pod;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "../data/gtk/pod_row.ui")]
    pub struct RflPodRow {
        #[template_child]
        pub(super) pod_name: TemplateChild<gtk::Label>,
        pub bindings: RefCell<Vec<Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RflPodRow {
        const NAME: &'static str = "RflPodRow";
        type Type = super::RflPodRow;
        type ParentType = gtk::Box;

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }
    }

    impl ObjectImpl for RflPodRow {}
    impl WidgetImpl for RflPodRow {}
    impl BoxImpl for RflPodRow {}
}

glib::wrapper! {
  pub struct RflPodRow(ObjectSubclass<imp::RflPodRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl RflPodRow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn bind(&self, obj: &Pod) {
        let name_label = self.imp().pod_name.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        let binding = obj
            .bind_property("pod-name", &name_label, "label")
            .sync_create()
            .build();

        bindings.push(binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
