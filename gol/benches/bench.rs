#![feature(test)]

extern crate gol;
extern crate test;

use gol::map;
use test::Bencher;

#[bench]
fn benchmark_with_acorn(b: &mut Bencher) {
    let mut map = map::Map::acorn();
    for _ in 0..4408 {
        map.next_generation();
    }
    b.iter(|| map.clone().next_generation());
}
