extern crate cairo;
extern crate futures_glib;
extern crate gdk;
extern crate gtk;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
#[macro_use]
extern crate serde_derive;
extern crate gol;
extern crate serde_json;

pub use gol::map;
mod life_reader;
mod view;

fn main() {
    let a = 1;
    std::env::set_var("GDK_BACKEND", "x11");
    relm::run::<view::Win>(()).unwrap();
}
