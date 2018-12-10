use croaring::Bitmap;
use std::cmp::Ordering::{self, Equal, Less, Greater};
use super::{FACTORIAL, CAPACITY, SYMMETRY};

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

    pub fn number_of_permutations(&self) -> usize {
        self.number_of_bits() - self.number_of_counter_bits()
    }

    pub fn number_of_bits(&self) -> usize {
        self.bitmap.cardinality() as usize
    }

    fn number_of_counter_bits(&self) -> usize {
        let range = (*FACTORIAL as u32)..*CAPACITY;
        range.filter(|b| self.bitmap.contains(*b)).count()
    }

    pub fn maximum_permutations() -> usize {
        *FACTORIAL as usize
    }
}

impl Eq for Candidate { }

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = &self.bitmap;
        let right = &other.bitmap;

        let left_len = left.cardinality();
        let right_len = right.cardinality();

        if left_len < right_len {
            return Less;
        }

        if left_len > right_len {
            return Greater;
        }

        for (a, b) in left.iter().zip(right.iter()) {
            match a.cmp(&b) {
                Equal => continue,
                o @ _ => return o,
            };
        }

        Equal
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test;
