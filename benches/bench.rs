#![feature(test)]
extern crate test;
extern crate leftpad;

use leftpad::leftpad;
use test::Bencher;

#[bench]
fn bench_leftpad(b: &mut Bencher) {
    b.iter(|| leftpad("", 10_000));
}
