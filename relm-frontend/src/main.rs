#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

mod view;

fn main() {
    std::env::set_var("GDK_BACKEND", "x11");
    relm::run::<crate::view::Win>(()).unwrap();
}