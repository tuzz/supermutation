#![feature(uniform_paths)]
#![feature(drain_filter)]

#[macro_use]
extern crate lazy_static;
extern crate croaring;
extern crate lehmer;

mod candidate;
mod closed_set;
mod heuristic;
mod open_set;
mod search;
mod symmetry;
mod util;

use util::Util;

lazy_static! {
    static ref N: usize = 5;
    static ref SYMBOLS: usize = *N - 1;
    static ref FACTORIAL: usize = Util::factorial(*N);
    static ref CAPACITY: u32 = (*FACTORIAL + *N - 2) as u32;
}

fn main() {
    println!("Hello, world!");
}
