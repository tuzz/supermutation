use croaring::Bitmap;
use std::cmp::Ordering::{self, Equal};
use super::{FACTORIAL, CAPACITY};
use super::symmetry::SYMMETRY;

#[derive(Clone, Debug, PartialEq)]
pub struct Candidate {
    bitmap: Bitmap,
}

impl Candidate {
    pub fn seed() -> Self {
        let mut bitmap = Bitmap::create_with_capacity(*CAPACITY);

        bitmap.add(0);

        for i in *FACTORIAL..(*CAPACITY as usize) {
            bitmap.add(i as u32);
        }

        bitmap.run_optimize();

        Self { bitmap }
    }

    pub fn expand(&self, symbol: usize) -> Self {
        let mut bitmap = Bitmap::create_with_capacity(*CAPACITY);
        let mapping = SYMMETRY.mapping(symbol, &self.bitmap);

        for bit in self.bitmap.iter() {
            bitmap.add(mapping[bit as usize]);
        }

        // Set the bit for the ground truth:
        bitmap.add(*FACTORIAL as u32);

        bitmap.run_optimize();

        Self { bitmap }
    }
}

impl Eq for Candidate { }

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = self.bitmap.iter();
        let right = other.bitmap.iter();

        let option = left.zip(right).find_map(|(a, b)| {
            match a.cmp(&b) {
                Equal => None,
                o @ _ => Some(o),
            }
        });

        option.unwrap_or(Equal)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test;
