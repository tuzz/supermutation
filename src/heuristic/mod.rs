#[derive(Clone, Debug, PartialEq)]
pub struct Heuristic {
    pub costs: Vec<usize>,
    pub changed_previous_values: bool,
}

impl Heuristic {
    pub fn new() -> Self {
        Self { costs: vec![1], changed_previous_values: true }
    }

    pub fn cost(&self, _number_of_bits: usize) -> usize {
        1
    }

    pub fn improve_based_on(&mut self, _: usize, _: usize) {
        // TODO
    }
}
