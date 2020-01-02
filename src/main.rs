extern crate gdk_pixbuf;
extern crate gtk;
#[macro_use]
extern crate glib;

use crate::util::load_pixbufs;
use booru::posts::Post;
use gio::prelude::*;
use gio::{ListModel, ListStore};
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, FlowBox, Image, SearchBar, SearchEntry};
use reqwest::Url;
use std::default::Default;
use std::env;
use std::env::args;
use std::iter::Iterator;
use std::option::Option;
use std::option::Option::Some;
use std::prelude::v1::Vec;
use std::result::Result;
use std::result::Result::{Err, Ok};
use std::string::String;

mod model;
mod util;

use post_data::PostData;
use std::clone::Clone;

const GLADE_SRC: &str = include_str!("grid.glade");

struct PostClient {
    client: booru::BooruClient,
}

impl PostClient {
    fn new() -> PostClient {
        drop(dotenv::dotenv());
        match envy::from_env::<booru::Config>() {
            Ok(config) => {
                println!("Parsed config.");
                env::set_var("API_KEY", config.api_key);
                env::set_var("LOGIN", config.login);
            }
            Err(e) => eprintln!("Failed to parse config file!\n{:#?}", e),
        }

        let login = env::var("LOGIN").expect("Failed to parse login");
        let api_key = env::var("API_KEY").expect("Failed to parse api key");

        PostClient {
            client: booru::BooruClient::new(login, api_key),
        }
    }

    fn search(&self, query: &str) -> Option<Vec<Post>> {
        match self.client.search_tag(query.to_string()) {
            Ok(o) => Some(o),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }
}

fn build_ui(application: &gtk::Application) -> std::io::Result<()> {
    let post_client = PostClient::new();
    let builder = Builder::new_from_string(GLADE_SRC);
    let client = reqwest::blocking::Client::new();

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let pc: FlowBox = builder
        .get_object("postcontainer")
        .expect("Couldn't get post container");

    let list_store = gio::ListStore::new(PostData::static_type());

    pc.bind_model(
        Some(&list_store),
        clone!(@weak window => @default-panic, move |item| {
          let box_ = gtk::ListBoxRow::new();
          let item = item.downcast_ref::<PostData>().expect("Row data is of wrong type");
          let vbox: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 5);

          if let Ok(large_file_url) = &item.get_property("preview_file_url").unwrap().downcast() {
            if let Some(url_string) = large_file_url.get() {
            let url = Url::parse(url_string).unwrap();
            let pixbuf = util::load_pixbufs(client.clone(), &[url], 400);
            let image = gtk::Image::new_from_pixbuf(pixbuf[0].as_ref());
            vbox.pack_start(&image, false, false, 0);
            }
          }

          box_.add(&vbox);
          box_.show_all();
          box_.upcast::<gtk::Widget>()
        }),
    );

    let search_entry: SearchEntry = builder
        .get_object("search_entry")
        .expect("Couldn't get search entry");

    search_entry.connect_activate({
        move |search_entry| {
            let query = search_entry.get_text();
            println!("Query: {:#?}", query);
            if let Some(query_text) = query {
                let posts = &post_client.search(&query_text);
                if let Some(results) = posts {
                    list_store.remove_all();
                    add_posts_to_model(results.to_vec(), list_store.clone());
                }
            }
        }
    });

    window.show_all();
    Ok(())
}

fn add_posts_to_model(posts: Vec<Post>, store: ListStore) {
    posts.iter().for_each(|p: &Post| {
        let pd = PostData::new(
            p.get_id(),
            p.get_image_width(),
            p.get_image_height(),
            &p.get_file_ext(),
            &p.get_file_url(),
            &p.get_large_file_url(),
            &p.get_preview_file_url(),
            &p.get_tag_string_artist(),
            &p.get_tag_string_character(),
            &p.get_tag_string_copyright(),
            &p.get_tag_string_general(),
            &p.get_tag_string_meta(),
        );
        &store.append(&pd);
    })
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app).unwrap();
    });
    application.run(&args().collect::<Vec<_>>());
}

mod post_data {
    use super::*;

    use glib::subclass;
    use glib::subclass::prelude::*;
    use glib::translate::*;

    // Implementation sub-module of the GObject
    mod imp {
        use super::*;
        use std::cell::RefCell;

        // The actual data structure that stores our values. This is not accessible
        // directly from the outside.
        #[derive(Debug)]
        pub struct PostData {
            id: RefCell<u64>,
            image_width: RefCell<u64>,
            image_height: RefCell<u64>,
            file_ext: RefCell<Option<String>>,
            file_url: RefCell<Option<String>>,
            large_file_url: RefCell<Option<String>>,
            preview_file_url: RefCell<Option<String>>,
            tag_string_artist: RefCell<Option<String>>,
            tag_string_character: RefCell<Option<String>>,
            tag_string_copyright: RefCell<Option<String>>,
            tag_string_general: RefCell<Option<String>>,
            tag_string_meta: RefCell<Option<String>>,
        }

        // GObject property definitions for our two values
        static PROPERTIES: [subclass::Property; 12] = [
            subclass::Property("id", |name| {
                glib::ParamSpec::uint64(
                    name,
                    "Id",
                    "Post Id",
                    0,
                    std::u64::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("image_width", |name| {
                glib::ParamSpec::uint64(
                    name,
                    "Image Width",
                    "Image Width",
                    0,
                    20000,
                    0,
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("image_height", |name| {
                glib::ParamSpec::uint64(
                    name,
                    "Image Height",
                    "Image Height",
                    0,
                    20000,
                    0,
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("file_ext", |name| {
                glib::ParamSpec::string(
                    name,
                    "File Ext",
                    "File Extension",
                    None,
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("file_url", |name| {
                glib::ParamSpec::string(
                    name,
                    "File Url",
                    "File URL",
                    None,
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("large_file_url", |name| {
                glib::ParamSpec::string(
                    name,
                    "Large File Url",
                    "Large File URL",
                    None,
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("preview_file_url", |name| {
                glib::ParamSpec::string(
                    name,
                    "Preview File Url",
                    "Preview File URL",
                    None,
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("tag_string_artist", |name| {
                glib::ParamSpec::string(
                    name,
                    "Tag String Artist",
                    "Tag String (Artist)",
                    None,
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("tag_string_character", |name| {
                glib::ParamSpec::string(
                    name,
                    "Tag String Character",
                    "Tag String (Character)",
                    None,
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("tag_string_copyright", |name| {
                glib::ParamSpec::string(
                    name,
                    "Tag String Copyright",
                    "Tag String (Copyright)",
                    None,
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("tag_string_general", |name| {
                glib::ParamSpec::string(
                    name,
                    "Tag String General",
                    "Tag String (General)",
                    None,
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("tag_string_meta", |name| {
                glib::ParamSpec::string(
                    name,
                    "Tag String Meta",
                    "Tag String (Meta)",
                    None,
                    glib::ParamFlags::READWRITE,
                )
            }),
        ];

        // Basic declaration of our type for the GObject type system
        impl ObjectSubclass for PostData {
            const NAME: &'static str = "PostData";
            type ParentType = glib::Object;
            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib_object_subclass!();

            // Called exactly once before the first instantiation of an instance. This
            // sets up any type-specific things, in this specific case it installs the
            // properties so that GObject knows about their existence and they can be
            // used on instances of our type
            fn class_init(klass: &mut Self::Class) {
                klass.install_properties(&PROPERTIES);
            }

            // Called once at the very beginning of instantiation of each instance and
            // creates the data structure that contains all our state
            fn new() -> Self {
                Self {
                    id: RefCell::new(0),
                    image_width: RefCell::new(0),
                    image_height: RefCell::new(0),
                    file_ext: RefCell::new(None),
                    file_url: RefCell::new(None),
                    large_file_url: RefCell::new(None),
                    preview_file_url: RefCell::new(None),
                    tag_string_artist: RefCell::new(None),
                    tag_string_character: RefCell::new(None),
                    tag_string_copyright: RefCell::new(None),
                    tag_string_general: RefCell::new(None),
                    tag_string_meta: RefCell::new(None),
                }
            }
        }

        // The ObjectImpl trait provides the setters/getters for GObject properties.
        // Here we need to provide the values that are internally stored back to the
        // caller, or store whatever new value the caller is providing.
        //
        // This maps between the GObject properties and our internal storage of the
        // corresponding values of the properties.
        impl ObjectImpl for PostData {
            glib_object_impl!();

            fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
                let prop = &PROPERTIES[id];

                match *prop {
                    subclass::Property("id", ..) => {
                        let id = value
                            .get_some()
                            .expect("type conformity checked by `Object::set_property`");
                        self.id.replace(id);
                    }

                    subclass::Property("image_width", ..) => {
                        let image_width = value
                            .get_some()
                            .expect("type conformity checked by `Object::set_property`");
                        self.image_width.replace(image_width);
                    }
                    subclass::Property("image_height", ..) => {
                        let image_height = value
                            .get_some()
                            .expect("type conformity checked by `Object::set_property`");
                        self.image_height.replace(image_height);
                    }
                    subclass::Property("file_ext", ..) => {
                        let file_ext = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.file_ext.replace(file_ext);
                    }
                    subclass::Property("file_url", ..) => {
                        let file_url = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.file_url.replace(file_url);
                    }
                    subclass::Property("large_file_url", ..) => {
                        let large_file_url = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.large_file_url.replace(large_file_url);
                    }

                    subclass::Property("preview_file_url", ..) => {
                        let preview_file_url = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.preview_file_url.replace(preview_file_url);
                    }

                    subclass::Property("tag_string_artist", ..) => {
                        let tag_string_artist = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.tag_string_artist.replace(tag_string_artist);
                    }

                    subclass::Property("tag_string_character", ..) => {
                        let tag_string_character = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.tag_string_character.replace(tag_string_character);
                    }

                    subclass::Property("tag_string_copyright", ..) => {
                        let tag_string_copyright = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.tag_string_copyright.replace(tag_string_copyright);
                    }

                    subclass::Property("tag_string_general", ..) => {
                        let tag_string_general = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.tag_string_general.replace(tag_string_general);
                    }

                    subclass::Property("tag_string_meta", ..) => {
                        let tag_string_meta = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.tag_string_meta.replace(tag_string_meta);
                    }

                    _ => unimplemented!(),
                }
            }

            fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
                let prop = &PROPERTIES[id];

                match *prop {
                    subclass::Property("id", ..) => Ok(self.id.borrow().to_value()),
                    subclass::Property("image_width", ..) => {
                        Ok(self.image_width.borrow().to_value())
                    }
                    subclass::Property("image_height", ..) => {
                        Ok(self.image_height.borrow().to_value())
                    }
                    subclass::Property("file_ext", ..) => Ok(self.file_ext.borrow().to_value()),
                    subclass::Property("file_url", ..) => Ok(self.file_url.borrow().to_value()),
                    subclass::Property("large_file_url", ..) => {
                        Ok(self.large_file_url.borrow().to_value())
                    }
                    subclass::Property("preview_file_url", ..) => {
                        Ok(self.preview_file_url.borrow().to_value())
                    }
                    subclass::Property("tag_string_artist", ..) => {
                        Ok(self.tag_string_artist.borrow().to_value())
                    }
                    subclass::Property("tag_string_character", ..) => {
                        Ok(self.tag_string_character.borrow().to_value())
                    }
                    subclass::Property("tag_string_copyright", ..) => {
                        Ok(self.tag_string_copyright.borrow().to_value())
                    }
                    subclass::Property("tag_string_general", ..) => {
                        Ok(self.tag_string_general.borrow().to_value())
                    }
                    subclass::Property("tag_string_meta", ..) => {
                        Ok(self.tag_string_meta.borrow().to_value())
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }

    // Public part of the PostData type. This behaves like a normal gtk-rs-style GObject
    // binding
    glib_wrapper! {
      pub struct PostData(Object < subclass::simple::InstanceStruct < imp::PostData >, subclass::simple::ClassStruct < imp::PostData >, PostDataClass >);

      match fn {
          get_type => || imp::PostData::get_type().to_glib(),
      }
    }

    // Constructor for new instances. This simply calls glib::Object::new() with
    // initial values for our two properties and then returns the new instance
    impl PostData {
        pub fn new(
            id: u64,
            image_width: u64,
            image_height: u64,
            file_ext: &str,
            file_url: &str,
            large_file_url: &str,
            preview_file_url: &str,
            tag_string_artist: &str,
            tag_string_character: &str,
            tag_string_copyright: &str,
            tag_string_general: &str,
            tag_string_meta: &str,
        ) -> PostData {
            glib::Object::new(
                Self::static_type(),
                &[
                    ("id", &id),
                    ("image_width", &image_width),
                    ("image_height", &image_height),
                    ("file_ext", &file_ext),
                    ("file_url", &file_url),
                    ("large_file_url", &large_file_url),
                    ("preview_file_url", &preview_file_url),
                    ("tag_string_artist", &tag_string_artist),
                    ("tag_string_character", &tag_string_character),
                    ("tag_string_copyright", &tag_string_copyright),
                    ("tag_string_general", &tag_string_general),
                    ("tag_string_meta", &tag_string_meta),
                ],
            )
            .expect("Failed to create post data")
            .downcast()
            .expect("Created post data is of wrong type")
        }
    }
}
