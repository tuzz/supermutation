use ::bucket_queue::*;
use crate::candidate::Candidate;

pub struct OpenSet {
    candidates: BucketQueue<BucketQueue<Vec<Candidate>>>,
}

impl OpenSet {
    pub fn new() -> Self {
        Self { candidates: BucketQueue::new() }
    }

    pub fn seed(candidate: Candidate) -> Self {
        let mut open_set = Self::new();
        open_set.add(candidate, 1, 0);

        open_set
    }

    pub fn add(&mut self, candidate: Candidate, f_cost: usize, g_cost: usize) {
        let bucket = self.candidates.bucket_for_adding(f_cost);
        bucket.push(candidate, g_cost);
    }

    pub fn next(&mut self) -> Option<(Candidate, usize)> {
        let f_cost = self.candidates.min_priority()?;
        let bucket = self.candidates.bucket_for_removing(f_cost)?;

        let g_cost = bucket.max_priority()?;
        let candidate = bucket.pop(g_cost)?;

        Some((candidate, g_cost))
    }

    pub fn len(&self) -> usize {
        self.candidates.len()
    }
}

#[cfg(test)]
mod test;
