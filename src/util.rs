use gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader, PixbufLoaderExt};
use reqwest::blocking::{Client, Request, Response};
use reqwest::Url;
use std::boxed::Box;
use std::convert::Into;
use std::io::{BufReader, Read};
use std::iter::Iterator;
use std::option::Option;
use std::prelude::v1::Vec;

/// Load images from urls, resizing to a certain width
pub fn load_pixbufs(client: Client, urls: &[Url], max_width: i32) -> Vec<Option<Pixbuf>> {
    let mut bytes = Vec::with_capacity(512);
    //  let stream = urls.
    urls.iter()
        .map(|url| {
            let loader: PixbufLoader = PixbufLoader::new();
            {
                let response = client.get(url.clone()).send().expect("fuckl");
                let mut reader = BufReader::new(response);
                bytes.clear();
                reader.read_to_end(&mut bytes).unwrap();
                loader.write(&bytes).unwrap();
                loader.close().unwrap();
            };
            if let Some(mut image) = loader.get_pixbuf() {
                let (width, height) = (image.get_width(), image.get_height());
                if width > max_width {
                    let (new_width, new_height) = (max_width, (height * max_width) / width);
                    image = image
                        .scale_simple(new_width, new_height, InterpType::Bilinear)
                        .unwrap();
                }
                Some(image)
            } else {
                None
            }
        })
        .collect()
}
