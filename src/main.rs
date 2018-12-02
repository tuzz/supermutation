#![feature(uniform_paths)]
#![feature(drain_filter)]

#[macro_use]
extern crate lazy_static;

mod candidate;
mod closed_set;
mod heuristic;
mod incremental;
mod interface;
mod open_set;
mod search;
mod symmetry;
mod utility;

use candidate::Candidate;
use closed_set::ClosedSet;
use heuristic::Heuristic;
use incremental::Incremental;
use interface::Interface;
use open_set::OpenSet;
use search::Search;
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
    let open_set = OpenSet::new();
    let closed_set = ClosedSet::new();
    let search = Search::new(open_set, closed_set);
    let heuristic = Heuristic::new();
    let candidate = Candidate::seed();
    let mut incremental = Incremental::new(heuristic, search);

    incremental.shortest_path(candidate, |distance, subgoal, search, _heuristic| {
        println!("The shortest path to {} is {}", subgoal, distance);
        println!("Open set: {}", search.open_set_len());
        println!("Closed set: {}", search.closed_set_len());
        println!();
    });
}
