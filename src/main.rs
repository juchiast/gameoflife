#![feature(unboxed_closures)]
#![feature(fn_traits)]

extern crate gtk;
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
