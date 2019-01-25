#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn benchmark_with_acorn(b: &mut Bencher) {
    let mut map = gol::Map::acorn();
    for _ in 0..4408 {
        map.next_generation();
    }
    b.iter(|| map.clone().next_generation());
}
