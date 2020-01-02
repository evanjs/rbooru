extern crate gdk_pixbuf;
extern crate gtk;
#[macro_use]
extern crate glib;

use crate::util::load_pixbufs;
use booru::posts::Post;
use gio::prelude::*;
use gio::{ListModel, ListStore};
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, FlowBox, Image, ScrolledWindow, SearchBar, SearchEntry};
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
use std::convert::From;
use std::sync::{Arc, Mutex};

const GLADE_SRC: &str = include_str!("grid.glade");

#[derive(Clone)]
struct PostClient {
    client: booru::BooruClient,
    page_number: Arc<Mutex<u64>>,
    search_text: Arc<Mutex<String>>,
}

impl PostClient {
    fn update_search_text(&self, query: &str) {
        *self.search_text.lock().unwrap() = query.to_string()
    }

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
            page_number: Arc::new(Mutex::new(1)),
            search_text: Arc::new(Mutex::new(std::string::String::from(""))),
        }
    }

    fn search(&self, query: &str) -> Option<Vec<Post>> {
        match self.client.search_tag(query.to_string(), None, None) {
            Ok(o) => Some(o),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }

    fn next_page(&mut self) -> Option<Vec<Post>> {
        match self.client.search_tag(
            self.search_text.lock().unwrap().to_string(),
            None,
            Some(*self.page_number.lock().unwrap() + 1),
        ) {
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

    let list_store = gio::ListStore::new(PostData::static_type());
    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let scroller: ScrolledWindow = builder
        .get_object("scroller")
        .expect("Couldn't get scroller");

    let scroller_client = post_client.clone();
    let scroller_store = list_store.clone();
    scroller.connect_edge_reached(move |_, dir| {
        if dir == gtk::PositionType::Bottom {
            let posts = scroller_client.clone().next_page();
            if let Some(results) = posts {
                add_posts_to_model(results.to_vec(), scroller_store.clone());
                *scroller_client.page_number.lock().unwrap() += 1;
            };
        }
    });

    let pc: FlowBox = builder
        .get_object("postcontainer")
        .expect("Couldn't get post container");

    pc.bind_model(
    Some(&list_store),
    clone!( @ weak window => @ default - panic,
        move |item| {
            let box_ = gtk::ListBoxRow::new();
            let item = item.downcast_ref::< PostData> ().expect("Row data is of wrong type");
            let vbox: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 5);

            if let Ok(large_file_url) = & item.get_property("preview_file_url").unwrap().downcast() {
                if let Some(url_string) = large_file_url.get() {
                    let url = Url::parse(url_string).unwrap();
                    let pixbuf = util::load_pixbufs(client.clone(), & [url], 400);
                    let image = gtk::Image::new_from_pixbuf(pixbuf[0].as_ref());
                    vbox.pack_start( &image, false, false, 0);
                }
            }

        box_.add( & vbox);
        box_.show_all();
        box_.upcast::< gtk::Widget > ()
        }),
  );

    let search_entry: SearchEntry = builder
        .get_object("search_entry")
        .expect("Couldn't get search entry");

    let search_client = post_client.clone();
    search_entry.connect_search_changed({
        move |search_entry| {
            let query = search_entry.clone().get_text();
            if let Some(query_text) = query {
                let text = &query_text.to_string();
                &mut search_client.update_search_text(text);
            }
        }
    });

    search_entry.connect_activate({
        move |search_entry| {
            let query = search_entry.clone().get_text();
            if let Some(query_text) = query {
                let text = &query_text.to_string();
                let posts = &post_client.search(text);
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

mod post_data;
