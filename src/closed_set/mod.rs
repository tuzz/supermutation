use crate::candidate::Candidate;
use std::collections::BTreeMap;

pub struct ClosedSet {
    candidates: BTreeMap<Candidate, usize>,
}

impl ClosedSet {
    pub fn new() -> Self {
        Self { candidates: BTreeMap::new() }
    }

    pub fn add(&mut self, candidate: Candidate, g_cost: usize) {
        self.candidates.insert(candidate, g_cost);
    }

    pub fn contains(&self, candidate: &Candidate, g_cost: usize) -> bool {
        match self.candidates.get(candidate) {
            Some(previous) if g_cost >= *previous => true,
            _ => false,
        }
    }

    pub fn len(&self) -> usize {
        self.candidates.len()
    }
}

#[cfg(test)]
mod test;
