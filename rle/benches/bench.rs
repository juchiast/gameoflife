#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn gun_p22p(b: &mut Bencher) {
    let life = include_str!("gun-p22p.rle");
    b.iter(|| rle::read_str(life).unwrap());
}
