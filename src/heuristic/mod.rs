#[derive(Clone, Debug)]
pub struct Heuristic {

}

impl Heuristic {
    pub fn new() -> Self {
        Self { }
    }

    pub fn cost(&self, number_of_bits: usize) -> usize {
        100 - number_of_bits // TODO: this is a very bad heuristic!
    }

    pub fn improve_based_on(&mut self, _: usize, _: usize) {
        // TODO
    }
}
