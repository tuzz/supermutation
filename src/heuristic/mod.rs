use crate::candidate::Candidate;

#[derive(Clone, Debug, PartialEq)]
pub struct Heuristic {
    starting_bits: usize,
    distances: Vec<usize>,
    lower_bounds: Vec<usize>,
}

impl Heuristic {
    pub fn new(starting_bits: usize, distances: Vec<usize>, lower_bounds: Vec<usize>) -> Self {
        Self { starting_bits, distances, lower_bounds }
    }

    pub fn seed() -> Self {
        let starting_bits = Candidate::seed().number_of_bits();
        let distances = vec![0];
        let lower_bounds = (0..=(starting_bits + 1)).rev().collect();

        Self::new(starting_bits, distances, lower_bounds)
    }

    pub fn cost(&self, number_of_bits: usize) -> usize {
        self.lower_bounds[number_of_bits]
    }

    pub fn lower_bounds(&self) -> &Vec<usize> {
        &self.lower_bounds
    }

    pub fn improve_based_on(&mut self, shortest_path_distance: usize) {
        self.distances.push(shortest_path_distance);
        self.lower_bounds.push(0);

        let lower_bound = self.maximum_lower_bound_on_the_distance_to_next_goal();

        for number_of_bits in 0..self.next_goal() {
            let distance_from_start = self.shortest_distance(number_of_bits);
            let distance_to_goal = lower_bound as isize - distance_from_start;

            self.lower_bounds[number_of_bits] = distance_to_goal as usize;
        }
    }

    fn maximum_lower_bound_on_the_distance_to_next_goal(&self) -> usize {
        let mut greatest_distance = 0;

        for number_of_bits in self.first_goal()..self.next_goal() {
            let needed_bits = self.next_goal() - number_of_bits;

            let distance_from_start = self.shortest_distance(number_of_bits);
            let distance_to_goal = self.shortest_distance_to_add(needed_bits);

            let total_distance = distance_from_start as usize + distance_to_goal;

            if total_distance > greatest_distance {
                greatest_distance = total_distance;
            }
        }

        greatest_distance
    }

    fn first_goal(&self) -> usize {
        self.starting_bits + 1
    }

    fn next_goal(&self) -> usize {
        self.lower_bounds.len() - 1
    }

    fn shortest_distance(&self, number_of_bits: usize) -> isize {
        if number_of_bits < self.starting_bits {
            return number_of_bits as isize - self.starting_bits as isize;
        }

        let bits_added = number_of_bits - self.starting_bits;
        self.shortest_distance_to_add(bits_added) as isize
    }

    fn shortest_distance_to_add(&self, bits_to_add: usize) -> usize {
        self.distances[bits_to_add]
    }
}

#[cfg(test)]
mod test;
