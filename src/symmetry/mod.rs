use lehmer::Lehmer;
use std::iter::once;
use super::util::Util;
use super::N;

struct Symmetry {
    pub mappings: Vec<Vec<Vec<u8>>>,
}

lazy_static! {
    static ref SYMMETRY: Symmetry = Symmetry::precompute(*N);
}

impl Symmetry {
    fn precompute(n: usize) -> Symmetry {
        Symmetry { mappings: Self::mappings(n) }
    }

    fn transpositions(n: usize) -> Vec<Vec<Vec<u8>>> {
        Self::map_transpositions(n, |t| t)
    }

    fn mappings(n: usize) -> Vec<Vec<Vec<u8>>> {
        let factorial = Util::factorial(n);

        Self::map_transpositions(n, |transposition| {
            let inverse = (0..n as u8).map(|i| {
                transposition.iter().position(|x| *x == i).unwrap() as u8
            }).collect::<Vec<u8>>();

            (0..factorial).map(|i| {
                let lehmer = Lehmer::from_decimal(i, n);
                let permutation = lehmer.to_permutation();

                let iterator = permutation.iter().map(|p| inverse[*p as usize]);
                let mapped = iterator.collect::<Vec<u8>>();

                Lehmer::from_permutation(&mapped).to_decimal() as u8
            }).collect()
        })
    }

    fn map_transpositions<F>(n: usize, f: F) -> Vec<Vec<Vec<u8>>>
        where F: Fn(Vec<u8>) -> Vec<u8>
    {
        (0..(n - 1)).map(|i| {
            let factorial = Util::factorial(i);

            (0..factorial).map(|j| {
                let lehmer = Lehmer::from_decimal(j, i);
                let permutation = lehmer.to_permutation();

                let head = permutation.iter().map(|h| *h);
                let middle = (i..n).skip(1).map(|t| t as u8);
                let tail = once(i as u8);

                f(head.chain(middle).chain(tail).collect())
            }).collect()
        }).collect()
    }
}

#[cfg(test)]
mod test;
