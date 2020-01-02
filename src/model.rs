// Our GObject subclass for carrying a name and count for the ListBox model
//
// Both name and count are stored in a RefCell to allow for interior mutability
// and are exposed via normal GObject properties. This allows us to use property
// bindings below to bind the values with what widgets display in the UI

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use glib::prelude::*;
use gtk::prelude::*;

//mod post_data {
//  use super::*;
//
//  use glib::subclass;
//  use glib::subclass::prelude::*;
//  use glib::translate::*;
//
//    // Implementation sub-module of the GObject
//  mod imp {
//    use super::*;
//    use std::cell::RefCell;
//
//    // The actual data structure that stores our values. This is not accessible
//    // directly from the outside.
//    pub struct PostData {
//      id: RefCell<u64>,
//      image_width: RefCell<u64>,
//      image_height: RefCell<u64>,
//      file_ext: RefCell<Option<String>>,
//      file_url: RefCell<Option<String>>,
//      large_file_url: RefCell<Option<String>>,
//      preview_file_url: RefCell<Option<String>>,
//      tag_string_artist: RefCell<Option<String>>,
//      tag_string_character: RefCell<Option<String>>,
//      tag_string_copyright: RefCell<Option<String>>,
//      tag_string_general: RefCell<Option<String>>,
//      tag_string_meta: RefCell<Option<String>>,
//    }
//
//    // GObject property definitions for our two values
//    static PROPERTIES: [subclass::Property; 12] = [
//      subclass::Property("id", |name| {
//        glib::ParamSpec::uint64(name, "Id", "Post Id", 0, 0, 0, glib::ParamFlags::READWRITE)
//      }),
//      subclass::Property("image_width", |name| {
//        glib::ParamSpec::uint64(
//          name,
//          "Image Width",
//          "Image Width",
//          0,
//          10000,
//          0,
//          glib::ParamFlags::READWRITE,
//        )
//      }),
//      subclass::Property("image_height", |name| {
//        glib::ParamSpec::uint64(
//          name,
//          "Image Height",
//          "Image Height",
//          0,
//          10000,
//          0,
//          glib::ParamFlags::READWRITE,
//        )
//      }),
//      subclass::Property("file_ext", |name| {
//        glib::ParamSpec::string(
//          name,
//          "File Ext",
//          "File Extension",
//          None,
//          glib::ParamFlags::READWRITE,
//        )
//      }),
//      subclass::Property("file_url", |name| {
//        glib::ParamSpec::string(
//          name,
//          "File Url",
//          "File URL",
//          None,
//          glib::ParamFlags::READWRITE,
//        )
//      }),
//      subclass::Property("large_file_url", |name| {
//        glib::ParamSpec::string(
//          name,
//          "Large File Url",
//          "Large File URL",
//          None,
//          glib::ParamFlags::READWRITE,
//        )
//      }),
//      subclass::Property("preview_file_url", |name| {
//        glib::ParamSpec::string(
//          name,
//          "Preview File Url",
//          "Preview File URL",
//          None,
//          glib::ParamFlags::READWRITE,
//        )
//      }),
//      subclass::Property("tag_string_artist", |name| {
//        glib::ParamSpec::string(
//          name,
//          "Tag String Artist",
//          "Tag String (Artist)",
//          None,
//          glib::ParamFlags::READWRITE,
//        )
//      }),
//      subclass::Property("tag_string_character", |name| {
//        glib::ParamSpec::string(
//          name,
//          "Tag String Character",
//          "Tag String (Character)",
//          None,
//          glib::ParamFlags::READWRITE,
//        )
//      }),
//      subclass::Property("tag_string_copyright", |name| {
//        glib::ParamSpec::string(
//          name,
//          "Tag String Copyright",
//          "Tag String (Copyright)",
//          None,
//          glib::ParamFlags::READWRITE,
//        )
//      }),
//      subclass::Property("tag_string_general", |name| {
//        glib::ParamSpec::string(
//          name,
//          "Tag String General",
//          "Tag String (General)",
//          None,
//          glib::ParamFlags::READWRITE,
//        )
//      }),
//      subclass::Property("tag_string_meta", |name| {
//        glib::ParamSpec::string(
//          name,
//          "Tag String Meta",
//          "Tag String (Meta)",
//          None,
//          glib::ParamFlags::READWRITE,
//        )
//      }),
//    ];
//
//    // Basic declaration of our type for the GObject type system
//    impl ObjectSubclass for PostData {
//      const NAME: &'static str = "PostData";
//      type ParentType = glib::Object;
//      type Instance = subclass::simple::InstanceStruct<Self>;
//      type Class = subclass::simple::ClassStruct<Self>;
//
//      glib_object_subclass!();
//
//      // Called exactly once before the first instantiation of an instance. This
//      // sets up any type-specific things, in this specific case it installs the
//      // properties so that GObject knows about their existence and they can be
//      // used on instances of our type
//      fn class_init(klass: &mut Self::Class) {
//        klass.install_properties(&PROPERTIES);
//      }
//
//      // Called once at the very beginning of instantiation of each instance and
//      // creates the data structure that contains all our state
//      fn new() -> Self {
//        Self {
//          id: RefCell::new(0),
//          image_width: RefCell::new(0),
//          image_height: RefCell::new(0),
//          file_ext: RefCell::new(None),
//          file_url: RefCell::new(None),
//          large_file_url: RefCell::new(None),
//          preview_file_url: RefCell::new(None),
//          tag_string_artist: RefCell::new(None),
//          tag_string_character: RefCell::new(None),
//          tag_string_copyright: RefCell::new(None),
//          tag_string_general: RefCell::new(None),
//          tag_string_meta: RefCell::new(None),
//        }
//      }
//    }
//
//    // The ObjectImpl trait provides the setters/getters for GObject properties.
//    // Here we need to provide the values that are internally stored back to the
//    // caller, or store whatever new value the caller is providing.
//    //
//    // This maps between the GObject properties and our internal storage of the
//    // corresponding values of the properties.
//    impl ObjectImpl for PostData {
//      glib_object_impl!();
//
//      fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
//        let prop = &PROPERTIES[id];
//
//        match *prop {
//          subclass::Property("id", ..) => {
//            let id = value
//              .get_some()
//              .expect("type conformity checked by `Object::set_property`");
//            self.id.replace(id);
//          }
//
//          subclass::Property("image_width", ..) => {
//            let image_width = value
//              .get_some()
//              .expect("type conformity checked by `Object::set_property`");
//            self.image_width.replace(image_width);
//          }
//          subclass::Property("image_height", ..) => {
//            let image_height = value
//              .get_some()
//              .expect("type conformity checked by `Object::set_property`");
//            self.image_height.replace(image_height);
//          }
//          subclass::Property("file_ext", ..) => {
//            let file_ext = value
//              .get()
//              .expect("type conformity checked by `Object::set_property`");
//            self.file_ext.replace(file_ext);
//          }
//          subclass::Property("file_url", ..) => {
//            let file_url = value
//              .get()
//              .expect("type conformity checked by `Object::set_property`");
//            self.file_url.replace(file_url);
//          }
//          subclass::Property("large_file_url", ..) => {
//            let large_file_url = value
//              .get()
//              .expect("type conformity checked by `Object::set_property`");
//            self.large_file_url.replace(large_file_url);
//          }
//
//          _ => unimplemented!(),
//        }
//      }
//
//      fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
//        let prop = &PROPERTIES[id];
//
//        match *prop {
//          subclass::Property("id", ..) => Ok(self.id.borrow().to_value()),
//          subclass::Property("image_width", ..) => {
//            Ok(self.image_width.borrow().to_value())
//          }
//          subclass::Property("image_height", ..) => {
//            Ok(self.image_height.borrow().to_value())
//          }
//          subclass::Property("file_ext", ..) => Ok(self.file_ext.borrow().to_value()),
//          subclass::Property("file_url", ..) => Ok(self.file_url.borrow().to_value()),
//          subclass::Property("large_file_url", ..) => Ok(self.large_file_url.borrow().to_value()),
//          subclass::Property("preview_file_url", ..) => Ok(self.preview_file_url.borrow().to_value()),
//          subclass::Property("tag_string_artist", ..) => Ok(self.tag_string_artist.borrow().to_value()),
//          subclass::Property("tag_string_character", ..) => Ok(self.tag_string_character.borrow().to_value()),
//          subclass::Property("tag_string_copyright", ..) => Ok(self.tag_string_copyright.borrow().to_value()),
//          subclass::Property("tag_string_general", ..) => Ok(self.tag_string_general.borrow().to_value()),
//          subclass::Property("tag_string_meta", ..) => Ok(self.tag_string_meta.borrow().to_value()),
//          _ => unimplemented!(),
//        }
//      }
//    }
//  }
//
//  // Public part of the PostData type. This behaves like a normal gtk-rs-style GObject
//  // binding
//  glib_wrapper! {
//        pub struct PostData(Object<subclass::simple::InstanceStruct<imp::PostData>, subclass::simple::ClassStruct<imp::PostData>, PostDataClass>);
//
//        match fn {
//            get_type => || imp::PostData::get_type().to_glib(),
//        }
//    }
//
//  // Constructor for new instances. This simply calls glib::Object::new() with
//  // initial values for our two properties and then returns the new instance
//  impl PostData {
//    pub fn new(
//      id: u64,
//      image_width: u64,
//      image_height: u64,
//      file_ext: &str,
//      file_url: &str,
//      large_file_url: &str,
//      preview_file_url: &str,
//      tag_string_artist: &str,
//      tag_string_character: &str,
//      tag_string_copyright: &str,
//      tag_string_general: &str,
//      tag_string_meta: &str,
//    ) -> PostData {
//      glib::Object::new(
//        Self::static_type(),
//        &[
//          ("id", &id),
//          ("image_width", &image_width),
//          ("image_height", &image_height),
//          ("file_ext", &file_ext),
//          ("file_url", &file_url),
//          ("large_file_url", &large_file_url),
//          ("preview_file_url", &preview_file_url),
//          ("tag_string_artist", &tag_string_artist),
//          ("tag_string_character", &tag_string_character),
//          ("tag_string_copyright", &tag_string_copyright),
//          ("tag_string_general", &tag_string_general),
//          ("tag_string_meta", &tag_string_meta),
//        ],
//      )
//        .expect("Failed to create post data")
//        .downcast()
//        .expect("Created post data is of wrong type")
//    }
//  }
//}
