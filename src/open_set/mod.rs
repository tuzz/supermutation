use ::bucket_queue::*;
use std::collections::HashMap;
use crate::candidate::Candidate;

pub struct OpenSet {
    pub candidates: BucketQueue<BucketQueue<Vec<Candidate>>>,
}

type IndexedBuckets = HashMap<usize, Vec<(Vec<Candidate>, usize)>>;

impl OpenSet {
    pub fn new() -> Self {
        Self { candidates: BucketQueue::new() }
    }

    pub fn seed(&mut self, candidate: Candidate) {
        self.add(candidate, 1, 0);
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

    pub fn buckets_indexed_by_h_cost(&mut self) -> IndexedBuckets {
        let mut index = HashMap::new();

        let f_min = self.minimum_f_cost().unwrap_or(0);
        let f_max = self.maximum_f_cost().unwrap_or(0);

        for f_cost in (f_min..=f_max).rev() {
            let mut f_bucket = self.candidates.bucket(f_cost);

            while let Some(g_cost) = f_bucket.min_priority() {
                let h_cost = f_cost - g_cost;

                if let Some(bucket) = f_bucket.replace(g_cost, None) {
                    index.entry(h_cost).or_insert(vec![]).push((bucket, g_cost));
                }
            }
        }

        index
    }

    pub fn reindex_by_h_cost(&mut self, index: &mut IndexedBuckets, old_h: usize, new_h: usize) -> Option<()> {
        for (mut bucket, g_cost) in index.remove(&old_h)? {
            let f_cost = g_cost + new_h;
            let mut f_bucket = self.candidates.bucket(f_cost);

            if !f_bucket.bucket(g_cost).is_empty() {
                let mut existing = f_bucket.replace(g_cost, None).unwrap();
                bucket.append(&mut existing);
            }

            f_bucket.replace(g_cost, Some(bucket));
        }

        None
    }
}

#[cfg(test)]
mod test;
