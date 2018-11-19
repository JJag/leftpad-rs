extern crate leftpad;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = &args[1];
    let n = args[2].parse().unwrap();
    println!("{}", leftpad::leftpad(s, n));
}
