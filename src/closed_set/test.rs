#![cfg(not(feature = "four_symbols"))]

use super::*;

type Subject = ClosedSet;

mod add {
    use super::*;

    #[test]
    fn it_adds_a_candidate_to_the_closed_set_with_its_g_cost() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed();

        subject.add(candidate.clone(), 0);

        assert_eq!(subject.candidates.len(), 1);
    }
}

mod contains {
    use super::*;

    #[test]
    fn it_returns_true_if_the_closed_set_contains_the_candidate() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed();

        let first = candidate.expand(0);
        let second = candidate.expand(1);

        subject.add(first.clone(), 0);

        assert_eq!(subject.contains(&first, 0), true);
        assert_eq!(subject.contains(&second, 0), false);
    }

    #[test]
    fn it_returns_false_if_it_contains_the_candidate_with_a_higher_g_cost() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed();

        subject.add(candidate.clone(), 5);

        assert_eq!(subject.contains(&candidate, 4), false);
        assert_eq!(subject.contains(&candidate, 5), true);
        assert_eq!(subject.contains(&candidate, 6), true);
    }
}

mod len {
    use super::*;

    #[test]
    fn it_returns_the_number_of_candidates_in_the_closed_set() {
        let candidate = Candidate::seed();

        let mut subject = Subject::new();
        assert_eq!(subject.len(), 0);

        subject.add(candidate, 0);
        assert_eq!(subject.len(), 1);
    }
}
