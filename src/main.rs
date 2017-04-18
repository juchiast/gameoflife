extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

mod map;
mod view;

fn main() {
    relm::run::<view::Win>().unwrap();
}
