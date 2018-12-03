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

    pub fn improve_based_on(&mut self, _: usize) {
        // TODO
    }

    pub fn lower_bounds(&self) -> &Vec<usize> {
        &self.lower_bounds
    }
}

#[cfg(test)]
mod test;
