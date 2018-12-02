use crate::candidate::Candidate;
use std::collections::BTreeSet;

pub struct ClosedSet {
    candidates: BTreeSet<Candidate>,
}

impl ClosedSet {
    pub fn new() -> Self {
        Self { candidates: BTreeSet::new() }
    }

    pub fn add(&mut self, candidate: Candidate) {
        self.candidates.insert(candidate);
    }

    pub fn contains(&self, candidate: &Candidate) -> bool {
        self.candidates.contains(candidate)
    }
}

#[cfg(test)]
mod test;
