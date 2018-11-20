#![feature(test)]
extern crate test;
extern crate leftpad;

use leftpad::leftpad;
use test::Bencher;

#[bench]
fn len_100(b: &mut Bencher) {
    b.iter(|| leftpad("", 100));
}

#[bench]
fn len_10_000(b: &mut Bencher) {
    b.iter(|| leftpad("", 10_000));
}
