#![feature(uniform_paths)]
#![feature(drain_filter)]

#[macro_use]
extern crate lazy_static;
extern crate croaring;
extern crate lehmer;

mod candidate;
mod closed_set;
mod heuristic;
mod interface;
mod open_set;
mod search;
mod symmetry;
mod utility;

use interface::Interface;
use symmetry::Symmetry;
use utility::Utility;

lazy_static! {
    static ref SYMBOLS: usize = Interface::ask_for_symbols();
    static ref EXPANSIONS: usize = *SYMBOLS - 1;
    static ref FACTORIAL: usize = Utility::factorial(*SYMBOLS);
    static ref CAPACITY: u32 = (*FACTORIAL + *SYMBOLS - 2) as u32;
    static ref SYMMETRY: Symmetry = Symmetry::precompute(*SYMBOLS);
}

fn main() {
    println!("Hello, world!");
}
