use crate::candidate::Candidate;
use crate::heuristic::Heuristic;
use crate::search::Search;

pub struct Incremental {
    heuristic: Heuristic,
    search: Search,
}

impl Incremental {
    pub fn new(heuristic: Heuristic, search: Search) -> Self {
        Self { heuristic, search }
    }

    pub fn shortest_path<F>(&mut self, candidate: Candidate, mut milestone: F) -> Option<usize>
        where F: FnMut(usize, usize, &Search, &Heuristic)
    {
        let search = &mut self.search;
        let heuristic = &mut self.heuristic;

        let start = candidate.number_of_bits();
        let finish = Candidate::maximum_bits();

        search.seed(candidate);
        let mut distance = None;

        for subgoal in start..=finish {
            distance = search.shortest_path(subgoal);
            heuristic.improve_based_on(distance?, subgoal);

            milestone(distance?, subgoal, &search, &heuristic);
            search.update_heuristic(heuristic); // TODO: only if changed
        }

        distance
    }
}

#[cfg(test)]
mod test;
