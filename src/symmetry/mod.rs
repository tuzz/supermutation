use croaring::Bitmap;
use lehmer::Lehmer;
use std::iter::{once, repeat};
use super::util::Util;
use super::N;

pub struct Symmetry {
    mappings: Vec<Vec<Vec<u32>>>,
}

lazy_static! {
    pub static ref SYMMETRY: Symmetry = Symmetry::precompute(*N);
}

impl Symmetry {
    fn precompute(n: usize) -> Symmetry {
        Symmetry { mappings: Self::combined_mappings(n) }
    }

    pub fn mapping(&self, symbol: usize, bitmap: &Bitmap) -> &Vec<u32> {
        let mappings = &self.mappings[symbol];

        if mappings.len() == 1 {
            return &mappings[0]
        }

        let mut choices: Vec<usize> = (0..mappings.len()).collect();

        for i in 0..mappings[0].len() {
            let bit_is_set = |c: &usize| {
                bitmap.contains(mappings[*c][i])
            };

            if !choices.iter().any(bit_is_set) {
                continue;
            }

            choices.drain_filter(|c| !bit_is_set(c));

            if choices.len() == 1 {
                break;
            }
        }

        &mappings[choices[0]]
    }

    fn combined_mappings(n: usize) -> Vec<Vec<Vec<u32>>> {
        let counters = Self::counter_mappings(n);

        Self::permutation_mappings(n, |symbol, mapping| {
            let head = mapping.iter().map(|h| *h as u32);
            let tail = counters[symbol].clone();

            head.chain(tail).collect()
        })
    }

    fn counter_mappings(n: usize) -> Vec<Vec<u32>> {
        let ground_truth = Util::factorial(n) as u32;

        let template = (1..(n - 2))
            .map(|i| ground_truth + i as u32)
            .chain(once(0))
            .collect::<Vec<u32>>();


        (0..(n - 1)).rev().enumerate().map(|(j, i)| {
            let head = template[0..i].iter().map(|t| *t);
            let tail = repeat(ground_truth).take(j);

            head.chain(tail).collect()
        }).collect()
    }

    fn permutation_mappings<F, T>(n: usize, f: F) -> Vec<Vec<Vec<T>>>
        where F: Fn(usize, Vec<u8>) -> Vec<T>
    {
        let factorial = Util::factorial(n);

        Self::transpositions(n, |symbol, transposition| {
            let inverse = (0..n as u8).map(|i| {
                transposition.iter().position(|x| *x == i).unwrap() as u8
            }).collect::<Vec<u8>>();

            let mappings = (0..factorial).map(|i| {
                let lehmer = Lehmer::from_decimal(i, n);
                let permutation = lehmer.to_permutation();

                let iterator = permutation.iter().map(|p| inverse[*p as usize]);
                let mapped = iterator.collect::<Vec<u8>>();

                Lehmer::from_permutation(&mapped).to_decimal() as u8
            }).collect();

            f(symbol, mappings)
        })
    }

    fn transpositions<F, T>(n: usize, f: F) -> Vec<Vec<Vec<T>>>
        where F: Fn(usize, Vec<u8>) -> Vec<T>
    {
        (0..(n - 1)).map(|symbol| {
            let factorial = Util::factorial(symbol);

            (0..factorial).map(|i| {
                let lehmer = Lehmer::from_decimal(i, symbol);
                let permutation = lehmer.to_permutation();

                let head = permutation.iter().map(|h| *h);
                let middle = (symbol..n).skip(1).map(|t| t as u8);
                let tail = once(symbol as u8);

                let mapping = head.chain(middle).chain(tail).collect();
                f(symbol, mapping)
            }).collect()
        }).collect()
    }
}

#[cfg(test)]
mod test;
