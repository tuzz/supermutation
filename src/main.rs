#![feature(uniform_paths)]

#[macro_use]
extern crate lazy_static;
extern crate croaring;
extern crate lehmer;

mod candidate;
mod symmetry;
mod util;

use util::Util;

lazy_static! {
    static ref N: usize = 5;
    static ref FACTORIAL: usize = Util::factorial(*N);
}

fn main() {
    println!("Hello, world!");
}
