use croaring::Bitmap;
use super::{N, FACTORIAL};

struct Candidate {
    bitmap: Bitmap,
}

lazy_static! {
    static ref CAPACITY: u32 = (*FACTORIAL + *N - 1) as u32;
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
}

#[cfg(test)]
mod test;
