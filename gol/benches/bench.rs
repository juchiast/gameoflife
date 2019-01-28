#![feature(test)]

extern crate test;
extern crate rle;

use test::Bencher;

#[bench]
fn acorn(b: &mut Bencher) {
    let mut map = gol::Map::acorn();
    for _ in 0..4408 {
        map.next_generation();
    }
    b.iter(|| map.clone().next_generation());
}

#[bench]
fn gun_p22p(b: &mut Bencher) {
    let map = rle::read_str(include_str!("gun-p22p.rle")).unwrap();
    b.iter(|| map.clone().next_generation());
}
