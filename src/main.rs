#![feature(uniform_paths)]
#![feature(drain_filter)]

#[macro_use]
extern crate lazy_static;
extern crate croaring;
extern crate lehmer;

mod candidate;
mod closed_set;
mod open_set;
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
