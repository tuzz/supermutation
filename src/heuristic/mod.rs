#[derive(Clone, Debug, PartialEq)]
pub struct Heuristic {
    pub lower_bounds: Vec<usize>,
}

impl Heuristic {
    pub fn new() -> Self {
        Self { lower_bounds: vec![5, 4, 3, 2, 1, 0] }
    }

    pub fn cost(&self, number_of_bits: usize) -> usize {
        self.lower_bounds[number_of_bits]
    }

    pub fn improve_based_on(&mut self, _: usize) {
        // TODO
    }
}
