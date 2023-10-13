use gtk::{
  glib::{self, clone},
  prelude::*,
  subclass::prelude::*,
  CompositeTemplate
};

mod imp {
  use super::*;

  #[derive(Debug, Default, CompositeTemplate)]
  #[template(file = "../data/gtk/pod_list.ui")]
  pub struct RflPodList {
    #[template_child]
    pub(super) list_view: TemplateChild<gtk::ListView>,

    // pub(super) pod_list: OnceCell<PodList>
  }

  #[glib::object_subclass]
  impl ObjectSubclass for RflPodList {
    const NAME: &'static str = "RflPodList";
    type Type = super::RflPodList;
    type ParentType = gtk::Widget;

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
      obj.init_template();
    }

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        klass.bind_template_instance_callbacks();
    }
  }

  impl ObjectImpl for RflPodList {
    fn constructed(&self) {
      self.parent_constructed();
      let obj = self.obj();



    }
  }

  impl WidgetImpl for RflPodList {
    fn realize(&self) {
        self.parent_realize();
        // for number in 0..=100 {
        //   self.list_box.append(&gtk::Label::new(Some(&number.to_string())));
        // }
    }
  }
}

glib::wrapper! {
  pub struct RflPodList(ObjectSubclass<imp::RflPodList>)
    @extends gtk::Widget,
    @implements gtk::Buildable;
}

#[gtk::template_callbacks]
impl RflPodList {
  pub fn new() -> Self {
    glib::Object::builder()
      .build()
  }
}