extern crate gdk_pixbuf;
extern crate gtk;

use crate::util::load_pixbufs;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, FlowBox, Image};
use reqwest::Url;
use std::default::Default;
use std::env::args;
use std::iter::Iterator;

mod util;

const GLADE_SRC: &str = include_str!("grid.glade");

fn build_ui(application: &gtk::Application) -> std::io::Result<()> {
    let builder = Builder::new_from_string(GLADE_SRC);
    let client = reqwest::blocking::Client::new();

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let pc: FlowBox = builder
        .get_object("postcontainer")
        .expect("Couldn't get post container");

    let image = [];

    let images = load_pixbufs(client, &image, 500);
    images.iter().for_each(|pixbuf| {
        let image = Image::new_from_pixbuf(pixbuf.as_ref());
        pc.add(&image);
    });

    window.show_all();
    Ok(())
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
