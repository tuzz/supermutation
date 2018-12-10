use crate::candidate::Candidate;

#[derive(Clone, Debug, PartialEq)]
pub struct Heuristic {
    starting_perms: usize,
    distances: Vec<usize>,
    lower_bounds: Vec<usize>,
    max_depths: Vec<usize>,
}

impl Heuristic {
    pub fn new(starting_perms: usize, distances: Vec<usize>, lower_bounds: Vec<usize>, max_depths: Vec<usize>) -> Self {
        Self { starting_perms, distances, lower_bounds, max_depths }
    }

    pub fn seed() -> Self {
        let starting_perms = Candidate::seed().number_of_permutations();
        let distances = vec![0];
        let lower_bounds = (0..=(starting_perms + 1)).rev().collect();
        let max_depths = (0..=(starting_perms + 1)).map(|_| 0).collect();

        Self::new(starting_perms, distances, lower_bounds, max_depths)
    }

    pub fn cost(&self, number_of_perms: usize, search_depth: usize) -> usize {
        let lower_bound = self.lower_bounds[number_of_perms];
        let max_depth = self.max_depths[number_of_perms];

        lower_bound + max_depth.saturating_sub(search_depth)
    }

    pub fn improve_based_on(&mut self, shortest_path_distance: usize) {
        self.distances.push(shortest_path_distance);
        self.lower_bounds.push(0);
        self.max_depths.push(0);

        let lower_bound = self.maximum_lower_bound_on_the_distance_to_next_goal();

        for number_of_perms in 0..self.next_goal() {
            let next_min_depth = self.shortest_distance(number_of_perms + 1);
            let max_depth = next_min_depth.unwrap_or(lower_bound).saturating_sub(1);

            self.max_depths[number_of_perms] = max_depth;

            if number_of_perms < self.starting_perms {
                let distance_to_start = self.starting_perms - number_of_perms;
                self.lower_bounds[number_of_perms] = lower_bound + distance_to_start;
            } else {
                self.lower_bounds[number_of_perms] = lower_bound - max_depth;
            }
        }
    }

    fn maximum_lower_bound_on_the_distance_to_next_goal(&self) -> usize {
        let mut greatest_distance = 0;

        for number_of_perms in self.first_goal()..self.next_goal() {
            let needed_perms = self.next_goal() - number_of_perms;

            let distance_from_start = self.shortest_distance(number_of_perms).unwrap();
            let distance_to_goal = self.shortest_distance_to_add(needed_perms).unwrap();

            let total_distance = distance_from_start as usize + distance_to_goal;

            if total_distance > greatest_distance {
                greatest_distance = total_distance;
            }
        }

        greatest_distance
    }

    fn first_goal(&self) -> usize {
        self.starting_perms + 1
    }

    fn next_goal(&self) -> usize {
        self.lower_bounds.len() - 1
    }

    fn shortest_distance(&self, number_of_perms: usize) -> Option<usize> {
        let perms_added = number_of_perms.saturating_sub(self.starting_perms);
        self.shortest_distance_to_add(perms_added)
    }

    fn shortest_distance_to_add(&self, perms_to_add: usize) -> Option<usize> {
        self.distances.get(perms_to_add).cloned()
    }
}

#[cfg(test)]
mod test;
