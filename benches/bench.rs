#![feature(test)]
extern crate leftpad;
extern crate test;

use leftpad::leftpad;
use test::Bencher;

#[bench]
fn bench_leftpad(b: &mut Bencher) {
    b.iter(|| leftpad("", 10_000));
}
