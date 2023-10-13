use gtk::{
    gio,
    glib::{self, clone, MainContext},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate, StringList,
};
use std::iter;

use k8s_openapi::api::core::v1::{Namespace, Pod};
use kube::{api::ListParams, Api, Client};

use crate::{spawn, spawn_tokio};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "../data/gtk/window.ui")]
    pub struct RflWindow {
        #[template_child]
        pub(super) namespaces: TemplateChild<StringList>,
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

    impl ObjectImpl for RflWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let (sender, receiver) = MainContext::channel::<Vec<String>>(glib::PRIORITY_DEFAULT);

            spawn_tokio!(async move {
                let client = Client::try_default().await.unwrap();
                let namespaces: Api<Namespace> = Api::all(client);
                let lp = ListParams::default();
                let ns = namespaces.list(&lp).await.unwrap();
                let mut names = Vec::new();

                for n in ns.items {
                    names.push(n.metadata.name.unwrap());
                }

                sender.send(names).unwrap();
            });

            receiver.attach(
                None,
                clone!(@weak self as window => @default-return glib::Continue(false), move |name| {
                  for n in name {
                    window.namespaces.append(&n);
                  }
                  glib::Continue(true)
                }),
            );
        }
    }
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
        glib::Object::builder().property("application", app).build()
    }
}
