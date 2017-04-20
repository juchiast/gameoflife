#![feature(fn_traits, unboxed_closures)]

extern crate gtk;
extern crate gdk;
extern crate cairo;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate tokio_core;

mod map;
mod view;

fn main() {
    relm::run::<view::Win>().unwrap();
}
