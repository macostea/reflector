use adw::{self, subclass::prelude::*};
use gtk::{
    gio,
    glib::{self, clone, MainContext},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate, DropDown, ListItem, NoSelection, SignalListItemFactory, StringList,
    StringObject,
};
use std::cell::RefCell;

use k8s_openapi::api::core::v1::{Namespace, Pod};
use kube::{api::ListParams, Api, Client};

use crate::{pod_row::RflPodRow, spawn_tokio};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "../data/gtk/window.ui")]
    pub struct RflWindow {
        #[template_child]
        pub(super) namespaces: TemplateChild<StringList>,
        #[template_child]
        pub(super) namespace_dropdown: TemplateChild<DropDown>,

        #[template_child]
        pub(super) list_view: TemplateChild<gtk::ListView>,

        pub pods: RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RflWindow {
        const NAME: &'static str = "RflWindow";
        type Type = super::RflWindow;
        type ParentType = adw::ApplicationWindow;

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

            let obj = self.obj();
            obj.setup_pods();
            obj.get_namespaces();
            obj.setup_callbacks();
            obj.setup_factory();
        }
    }
    impl WidgetImpl for RflWindow {}
    impl WindowImpl for RflWindow {}
    impl AdwApplicationWindowImpl for RflWindow {}
    impl ApplicationWindowImpl for RflWindow {}
}

glib::wrapper! {
  pub struct RflWindow(ObjectSubclass<imp::RflWindow>)
    @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks]
impl RflWindow {
    pub fn new<A: IsA<gtk::Application>>(app: &A) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn get_namespaces(&self) {
        let (sender, receiver) = MainContext::channel::<Vec<String>>(glib::Priority::DEFAULT);

        spawn_tokio!(async move {
            let client = Client::try_default().await.unwrap();
            let namespaces: Api<Namespace> = Api::all(client);
            let lp = ListParams::default();
            let ns = namespaces.list(&lp).await.unwrap();
            let mut names = Vec::new();

            for n in ns {
                names.push(n.metadata.name.unwrap());
            }

            sender.send(names).unwrap();
        });

        receiver.attach(
            None,
            clone!(@weak self as window => @default-return glib::ControlFlow::Break, move |name| {
              for n in name {
                window.imp().namespaces.append(&n);
              }
              glib::ControlFlow::Continue
            }),
        );
    }

    fn pods(&self) -> gio::ListStore {
        self.imp()
            .pods
            .borrow()
            .clone()
            .expect("Could not get pods")
    }

    fn setup_pods(&self) {
        let model = gio::ListStore::new::<crate::models::pod::Pod>();

        self.imp().pods.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.pods()));
        self.imp().list_view.set_model(Some(&selection_model));
    }

    fn setup_callbacks(&self) {
        self.imp().namespace_dropdown.connect_notify_local(
            Some("selected-item"),
            clone!(@weak self as window => move |drop_down, _| {
              let item = drop_down.selected_item().and_downcast::<StringObject>().unwrap();
              window.get_pods_for_namespace(item.string().to_string());
              println!("Selected item: {:?}", item.string());
            }),
        );
    }

    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let pod_row = RflPodRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be a ListItem")
                .set_child(Some(&pod_row));
        });

        factory.connect_bind(move |_, list_item| {
            let obj = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be a ListItem")
                .item()
                .and_downcast::<crate::models::pod::Pod>()
                .expect("Needs to be a Pod");

            let pod_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be a ListItem")
                .child()
                .and_downcast::<RflPodRow>()
                .expect("Needs to be a RflPodRow");

            pod_row.bind(&obj);
        });

        factory.connect_unbind(move |_, list_item| {
            let pod_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be a ListItem")
                .child()
                .and_downcast::<RflPodRow>()
                .expect("Needs to be a RflPodRow");

            pod_row.unbind();
        });

        self.imp().list_view.set_factory(Some(&factory));
    }

    fn add_pod(&self, name: String) {
        let pod = crate::models::pod::Pod::new(name);
        self.pods().append(&pod);
    }

    fn get_pods_for_namespace(&self, namespace: String) {
        self.pods().remove_all();
        let (sender, receiver) = MainContext::channel::<Vec<String>>(glib::Priority::DEFAULT);

        spawn_tokio!(async move {
            let client = Client::try_default().await.unwrap();
            let pods = Api::<Pod>::namespaced(client, &namespace);
            let lp = ListParams::default();
            let ps = pods.list(&lp).await.unwrap();
            let mut names = Vec::new();

            for p in ps {
                names.push(p.metadata.name.unwrap());
            }

            sender.send(names).unwrap();
        });

        receiver.attach(
            None,
            clone!(@weak self as window => @default-return glib::ControlFlow::Break, move |name| {
              for n in name {
                window.add_pod(n);
              }
              glib::ControlFlow::Continue
            }),
        );
    }
}
