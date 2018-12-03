use crate::candidate::Candidate;

#[derive(Clone, Debug, PartialEq)]
pub struct Heuristic {
    pub starting_bits: usize,
    pub lower_bounds: Vec<usize>,
    pub distances: Vec<usize>,
    pub invalidated: bool,
}

impl Heuristic {
    pub fn new() -> Self {
        let starting_bits = Candidate::seed().number_of_bits();
        let lower_bounds = (0..(starting_bits + 2)).rev().collect();
        let distances =  vec![];
        let invalidated = true;

        Self { starting_bits, lower_bounds, distances, invalidated }
    }

    pub fn cost(&self, number_of_bits: usize) -> usize {
        self.lower_bounds[number_of_bits]
    }

    pub fn improve_based_on(&mut self, shortest_path_distance: usize) {
        let next_goal = self.starting_bits + self.distances.len() + 1;
        self.distances.push(shortest_path_distance);
        println!("{:?}", self.distances);

        println!("next goal is {} bits", next_goal);

        for previous_goal in 0..next_goal {
            let bits_needed = next_goal - previous_goal;
            //println!("from {} bits i need to add {} bits", previous_goal, bits_needed);
            let lower_bound = self.shortest_distance(bits_needed);
            //println!("... and the shortest path to add {} bits is {}", bits_needed, lower_bound);

            self.lower_bounds[previous_goal] = lower_bound;
        }

        if next_goal == 6 {
        //panic!("");
        }

        self.lower_bounds.push(0);
    }

    fn shortest_distance(&self, additional_bits: usize) -> usize {
        if additional_bits < self.distances.len() {
            return self.distances[additional_bits]
        }

        self.distances.last().unwrap() + additional_bits - self.distances.len() + 1
    }
}

#[cfg(test)]
mod test;
