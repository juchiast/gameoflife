extern crate cairo;
extern crate futures_glib;
extern crate gdk;
extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod map;
mod view;

fn main() {
    std::env::set_var("GDK_BACKEND", "x11");
    relm::run::<view::Win>(()).unwrap();
}
