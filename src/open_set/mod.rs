use ::bucket_queue::*;
use crate::candidate::Candidate;

pub struct OpenSet {
    candidates: BucketQueue<BucketQueue<Vec<Candidate>>>,
}

impl OpenSet {
    pub fn new() -> Self {
        Self { candidates: BucketQueue::new() }
    }

    pub fn add(&mut self, candidate: Candidate, f_cost: usize, g_cost: usize) {
        let bucket = self.candidates.bucket_for_adding(f_cost);
        bucket.push(candidate, g_cost);
    }

    pub fn next(&mut self) -> Option<(Candidate, usize)> {
        let f_cost = self.minimum_f_cost()?;
        let bucket = self.candidates.bucket_for_removing(f_cost)?;

        let g_cost = bucket.max_priority()?;
        let candidate = bucket.pop(g_cost)?;

        Some((candidate, g_cost))
    }

    pub fn len(&self) -> usize {
        self.candidates.len()
    }

    pub fn minimum_f_cost(&self) -> Option<usize> {
        self.candidates.min_priority()
    }

    pub fn maximum_f_cost(&self) -> Option<usize> {
        self.candidates.max_priority()
    }
}

#[cfg(test)]
mod test;
