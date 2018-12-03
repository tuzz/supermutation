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
        Self { open_set, closed_set, heuristic: Heuristic::new() }
    }

    pub fn seed(&mut self, candidate: Candidate) {
        self.open_set.seed(candidate);
    }

    pub fn shortest_path(&mut self, goal: usize) -> Option<usize> {
        let open_set = &mut self.open_set;
        let closed_set = &mut self.closed_set;
        let mut reached_goal = false;

        while let Some((candidate, search_depth)) = open_set.next() {
            if closed_set.contains(&candidate) {
                continue;
            }

            if candidate.number_of_bits() == goal {
                reached_goal = true;
            }

            for symbol in 0..*EXPANSIONS {
                let neighbor = candidate.expand(symbol);

                if closed_set.contains(&neighbor) {
                    continue;
                }

                let bits = neighbor.number_of_bits();

                let g_cost = search_depth + 1;
                let h_cost = self.heuristic.cost(bits);
                let f_cost = g_cost + h_cost;
//                println!("neigbor: {:?}", neighbor);
//                println!("f: {}, g: {}, h: {}", f_cost, g_cost, h_cost);

                open_set.add(neighbor, f_cost, g_cost);
            }

            closed_set.add(candidate);

            if reached_goal {
                return Some(search_depth);
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
        if heuristic.invalidated {
            self.recalculate_open_set_costs(heuristic);
        }

        self.heuristic = heuristic.clone();
    }

    fn recalculate_open_set_costs(&mut self, heuristic: &Heuristic) {
        let mut index = self.open_set.buckets_indexed_by_h_cost();

        let old_costs = self.heuristic.lower_bounds.iter();
        let new_costs = heuristic.lower_bounds.iter();

        for (old_h, new_h) in old_costs.zip(new_costs) {
            self.open_set.reindex_by_h_cost(&mut index, *old_h, *new_h);
        }
    }
}

#[cfg(test)]
mod test;
