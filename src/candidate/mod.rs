use croaring::Bitmap;
use super::{N, FACTORIAL};
use super::symmetry::SYMMETRY;

struct Candidate {
    bitmap: Bitmap,
}

lazy_static! {
    static ref CAPACITY: u32 = (*FACTORIAL + *N - 2) as u32;
}

impl Candidate {
    fn seed() -> Self {
        let mut bitmap = Bitmap::create_with_capacity(*CAPACITY);

        bitmap.add(0);

        for i in *FACTORIAL..(*CAPACITY as usize) {
            bitmap.add(i as u32);
        }

        bitmap.run_optimize();

        Self { bitmap }
    }

    fn expand(&self, symbol: usize) -> Self {
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

#[cfg(test)]
mod test;
