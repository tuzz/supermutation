use lehmer::Lehmer;
use super::*;

type Subject = Candidate;

const T: bool = true;
const F: bool = false;

fn seen_permutation(subject: &Subject, slice: &[u8]) -> bool {
    let lehmer = Lehmer::from_permutation(slice);
    let decimal = lehmer.to_decimal() as u32;

    subject.bitmap.contains(decimal)
}

fn number_of_permutations(subject: &Subject) -> usize {
    (0..*FACTORIAL).filter(|i| subject.bitmap.contains(*i as u32)).count()
}

fn counter_bits(subject: &Subject) -> Vec<bool> {
    let range = (*FACTORIAL as u32)..*CAPACITY;
    range.map(|i| subject.bitmap.contains(i)).collect()
}

mod seed {
    use super::*;

    #[test]
    fn it_has_seen_the_first_permutation() {
        let subject = Subject::seed();

        assert_eq!(seen_permutation(&subject, &[0, 1, 2, 3, 4]), true);
        assert_eq!(seen_permutation(&subject, &[0, 1, 2, 4, 3]), false);
    }

    #[test]
    fn it_sets_all_the_counter_bits() {
        let subject = Subject::seed();

        assert_eq!(counter_bits(&subject), &[T, T, T]);
    }
}

mod expand {
    use super::*;

    #[test]
    fn it_maps_the_permutation_to_its_canonical_form() {
        let subject = Subject::seed();

        // For the 12340 transposition: 01234 -> 40123
        assert_eq!(seen_permutation(&subject.expand(0), &[4, 0, 1, 2, 3]), true);

        // For the *2341 transposition: 01234 -> 04123
        assert_eq!(seen_permutation(&subject.expand(1), &[0, 4, 1, 2, 3]), true);

        // For the **342 transposition: 01234 -> { 01423, 10423 }
        // 01423 is chosen because it's before 10423 in the bitmap
        assert_eq!(seen_permutation(&subject.expand(2), &[0, 1, 4, 2, 3]), true);
        assert_eq!(seen_permutation(&subject.expand(2), &[1, 0, 4, 2, 3]), false);

        // For the ***43 transposition: 01234 -> { 01243, 02143, 10243, 20143, 12043, 21043 }
        // 01243 is chosen because it's before the others in the bitmap
        assert_eq!(seen_permutation(&subject.expand(3), &[0, 1, 2, 4, 3]), true);
        assert_eq!(seen_permutation(&subject.expand(3), &[0, 2, 1, 4, 3]), false);
        assert_eq!(seen_permutation(&subject.expand(3), &[1, 0, 2, 4, 3]), false);
        assert_eq!(seen_permutation(&subject.expand(3), &[2, 0, 1, 4, 3]), false);
        assert_eq!(seen_permutation(&subject.expand(3), &[1, 2, 0, 4, 3]), false);
        assert_eq!(seen_permutation(&subject.expand(3), &[2, 1, 0, 4, 3]), false);
    }

    #[test]
    fn it_adds_a_new_permutation_when_the_zero_symbol_is_expanded() {
        let subject = Subject::seed();

        assert_eq!(number_of_permutations(&subject.expand(0)), 2);
        assert_eq!(number_of_permutations(&subject.expand(1)), 1);
        assert_eq!(number_of_permutations(&subject.expand(2)), 1);
        assert_eq!(number_of_permutations(&subject.expand(3)), 1);

        assert_eq!(seen_permutation(&subject.expand(0), &[0, 1, 2, 3, 4]), true);
    }

    #[test]
    fn it_incrementally_adds_new_permutations() {
        let subject = Subject::seed(); // 01234

        // The comments use the un-canonicalised form for simplicity.

        let candidate = subject.expand(0); // 012340
        assert_eq!(number_of_permutations(&candidate), 2);

        let candidate = candidate.expand(0); // 0123401
        assert_eq!(number_of_permutations(&candidate), 3);

        let candidate = candidate.expand(0); // 01234012
        assert_eq!(number_of_permutations(&candidate), 4);

        let candidate = candidate.expand(0); // 012340123
        assert_eq!(number_of_permutations(&candidate), 5);

        let candidate = candidate.expand(0); // 0123401234
        assert_eq!(number_of_permutations(&candidate), 5); // <-- the same

        let candidate = candidate.expand(1); // 01234012341
        assert_eq!(number_of_permutations(&candidate), 5);

        let candidate = candidate.expand(0); // 012340123410
        assert_eq!(number_of_permutations(&candidate), 6);

        let candidate = candidate.expand(1); // 0123401234103
        assert_eq!(number_of_permutations(&candidate), 6);

        let candidate = candidate.expand(0); // 01234012341032
        assert_eq!(number_of_permutations(&candidate), 7);

        let candidate = candidate.expand(2); // 012340123410320
        assert_eq!(number_of_permutations(&candidate), 7);

        let candidate = candidate.expand(1); // 0123401234103204
        assert_eq!(number_of_permutations(&candidate), 7);

        let candidate = candidate.expand(0); // 01234012341032041
        assert_eq!(number_of_permutations(&candidate), 8);
    }

    #[test]
    fn it_sets_counter_bits_based_on_the_expanded_symbol() {
        let subject = Subject::seed();

        assert_eq!(counter_bits(&subject.expand(0)), &[T, T, T]);
        assert_eq!(counter_bits(&subject.expand(1)), &[T, T, T]);
        assert_eq!(counter_bits(&subject.expand(2)), &[T, T, F]);
        assert_eq!(counter_bits(&subject.expand(3)), &[T, F, F]);
    }

    #[test]
    fn it_adds_removed_counter_bits_based_on_the_expanded_symbol() {
        let subject = Subject::seed();

        let candidate = subject.expand(2); // one bit removed
        assert_eq!(counter_bits(&candidate.expand(0)), &[T, T, T]);
        assert_eq!(counter_bits(&candidate.expand(1)), &[T, T, T]);
        assert_eq!(counter_bits(&candidate.expand(2)), &[T, T, F]);
        assert_eq!(counter_bits(&candidate.expand(3)), &[T, F, F]);

        let candidate = subject.expand(3); // two bits removed
        assert_eq!(counter_bits(&candidate.expand(0)), &[T, T, F]);
        assert_eq!(counter_bits(&candidate.expand(1)), &[T, T, F]);
        assert_eq!(counter_bits(&candidate.expand(2)), &[T, T, F]);
        assert_eq!(counter_bits(&candidate.expand(3)), &[T, F, F]);
    }
}

mod number_of_bits {
    use super::*;

    #[test]
    fn it_returns_the_number_of_bits_that_are_set_in_the_bitmap() {
        let subject = Subject::seed();
        let first = subject.expand(0);
        let second = subject.expand(1);
        let third = subject.expand(2);
        let fourth = subject.expand(3);

        assert_eq!(subject.number_of_bits(), 4); // 1 perm, 3 counters
        assert_eq!(first.number_of_bits(), 5);   // 2 perms, 3 counters
        assert_eq!(second.number_of_bits(), 4);  // 1 perm, 3 counters
        assert_eq!(third.number_of_bits(), 3);   // 1 perm, 2 counters
        assert_eq!(fourth.number_of_bits(), 2);  // 1 perm, 1 counters
    }
}

mod maximum_bits {
    use super::*;

    #[test]
    fn it_returns_the_capacity_of_the_bitmap() {
        assert_eq!(Subject::maximum_bits(), *CAPACITY as usize);
    }
}

mod ord {
    use super::*;

    #[test]
    fn it_consistently_orders_candidates() {
        let subject = Subject::seed();

        let a = subject.expand(0);
        let b = subject.expand(1);
        let c = subject.expand(2);
        let d = subject.expand(0);

        // Order is total and antisymmetric:
        assert!(a < b);
        assert!(!(a == b));
        assert!(!(a > b));

        // Order is transistive:
        assert!(a < c);
        assert!(c < b);
        assert!(a < b);

        // Order is consistent for the same bitmap:
        assert!(a == d);
        assert!(!(a < d));
        assert!(!(a > d));
    }
}
