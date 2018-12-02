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
mod utility;

use symmetry::Symmetry;
use utility::Utility;

lazy_static! {
    static ref N: usize = 5;
    static ref SYMBOLS: usize = *N - 1;
    static ref FACTORIAL: usize = Utility::factorial(*N);
    static ref CAPACITY: u32 = (*FACTORIAL + *N - 2) as u32;
    static ref SYMMETRY: Symmetry = Symmetry::precompute(*N);
}

fn main() {
    println!("Hello, world!");
}
