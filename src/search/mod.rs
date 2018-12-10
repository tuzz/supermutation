use super::EXPANSIONS;
use crate::candidate::Candidate;
use crate::closed_set::ClosedSet;
use crate::open_set::OpenSet;
use crate::heuristic::Heuristic;

pub struct Search {
    open_set: OpenSet,
    closed_set: ClosedSet,
    heuristic: Heuristic,
}

impl Search {
    pub fn new(open_set: OpenSet, closed_set: ClosedSet) -> Self {
        Self { open_set, closed_set, heuristic: Heuristic::seed() }
    }

    pub fn seed(&mut self, candidate: Candidate) {
        self.open_set.add(candidate, 1, 0);
    }

    pub fn shortest_path(&mut self, goal: usize) -> Option<usize> {
        let open_set = &mut self.open_set;
        let closed_set = &mut self.closed_set;
        let mut reached_goal = false;

        while let Some((candidate, search_depth)) = open_set.next() {
            if closed_set.contains(&candidate, search_depth) {
                continue;
            }

            for symbol in 0..*EXPANSIONS {
                let neighbor = candidate.expand(symbol);

                if closed_set.contains(&neighbor, search_depth + 1) {
                    continue;
                }

                let perms = neighbor.number_of_permutations();

                if perms == goal {
                    reached_goal = true;
                }

                let g_cost = search_depth + 1;
                let h_cost = self.heuristic.cost(perms, g_cost);
                let f_cost = g_cost + h_cost;

                open_set.add(neighbor, f_cost, g_cost);
            }

            closed_set.add(candidate, search_depth);

            if reached_goal {
                return Some(search_depth + 1);
            }
        }

        None
    }

    pub fn open_set_len(&self) -> usize {
        self.open_set.len()
    }

    pub fn closed_set_len(&self) -> usize {
        self.closed_set.len()
    }

    pub fn update_heuristic(&mut self, heuristic: &Heuristic) {
        let mut stack = vec![];

        while let Some(item) = self.open_set.next() {
            stack.push(item);
        }

        while let Some((candidate, g_cost)) = stack.pop() {
            let perms = candidate.number_of_permutations();

            let h_cost = heuristic.cost(perms, g_cost);
            let f_cost = g_cost + h_cost;

            self.open_set.add(candidate, f_cost, g_cost);
        }

        self.heuristic = heuristic.clone();
    }
}

#[cfg(test)]
mod test;
