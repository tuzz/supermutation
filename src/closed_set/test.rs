#![cfg(not(feature = "four_symbols"))]

use super::*;

type Subject = ClosedSet;

mod add {
    use super::*;

    #[test]
    fn it_adds_a_candidate_to_the_closed_set() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed();

        subject.add(candidate.clone());

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

        subject.add(first.clone());

        assert_eq!(subject.contains(&first), true);
        assert_eq!(subject.contains(&second), false);
    }
}

mod len {
    use super::*;

    #[test]
    fn it_returns_the_number_of_candidates_in_the_closed_set() {
        let candidate = Candidate::seed();

        let mut subject = Subject::new();
        assert_eq!(subject.len(), 0);

        subject.add(candidate);
        assert_eq!(subject.len(), 1);
    }
}
