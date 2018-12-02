use super::*;

type Subject = OpenSet;

mod new {
    use super::*;

    #[test]
    fn it_builds_an_empty_open_set() {
        let subject = Subject::new();
        assert_eq!(subject.candidates.len(), 0);
    }
}

mod seed {
    use super::*;

    #[test]
    fn it_builds_an_open_set_that_contains_the_candidate() {
        let candidate = Candidate::seed();
        let subject = Subject::seed(candidate);

        assert_eq!(subject.candidates.len(), 1);
    }

    #[test]
    fn it_sets_the_g_cost_to_zero_because_we_havent_travelled_anywhere_yet() {
        let candidate = Candidate::seed();
        let mut subject = Subject::seed(candidate);

        let bucket = subject.candidates.min_bucket();
        let g_cost = bucket.min_priority().unwrap();

        assert_eq!(g_cost, 0);
    }

    #[test]
    fn it_sets_the_f_cost_to_one_because_we_have_to_travel_at_least_once() {
        let candidate = Candidate::seed();
        let subject = Subject::seed(candidate);

        let f_cost = subject.candidates.min_priority().unwrap();

        assert_eq!(f_cost, 1);
    }
}

mod add {
    use super::*;

    #[test]
    fn it_adds_the_candidate_and_sets_their_f_cost() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed();

        subject.add(candidate, 12, 34);

        let f_cost = subject.candidates.min_priority().unwrap();

        assert_eq!(f_cost, 12);
    }

    #[test]
    fn it_adds_the_candidate_and_sets_their_g_cost() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed();

        subject.add(candidate, 12, 34);

        let bucket = subject.candidates.min_bucket();
        let g_cost = bucket.min_priority().unwrap();

        assert_eq!(g_cost, 34);
    }
}

mod next {
    use super::*;

    #[test]
    fn it_returns_the_candidates_ordered_by_f_cost_ascending() {
        let mut subject = Subject::new();
        let candidate = Candidate::seed();

        let first = candidate.expand(0);
        let second = candidate.expand(1);

        subject.add(first.clone(), 12, 34);
        subject.add(second.clone(), 56, 78);

        assert_eq!(subject.next(), Some((first, 34)));
        assert_eq!(subject.next(), Some((second, 78)));
        assert_eq!(subject.next(), None);
    }

    mod when_candidates_have_the_same_f_cost {
        use super::*;

        #[test]
        fn it_returns_the_candidates_ordered_by_g_cost_descending() {
            let mut subject = Subject::new();
            let candidate = Candidate::seed();

            let first = candidate.expand(0);
            let second = candidate.expand(1);
            let third = candidate.expand(2);

            subject.add(first.clone(), 12, 34);
            subject.add(second.clone(), 56, 78);
            subject.add(third.clone(), 12, 33);

            assert_eq!(subject.next(), Some((first, 34)));
            assert_eq!(subject.next(), Some((third, 33)));
            assert_eq!(subject.next(), Some((second, 78)));
        }
    }
}

mod len {
    use super::*;

    #[test]
    fn it_returns_the_number_of_candidates_in_the_open_set() {
        let candidate = Candidate::seed();

        let mut subject = Subject::new();
        assert_eq!(subject.len(), 0);

        subject.add(candidate, 12, 34);
        assert_eq!(subject.len(), 1);
    }
}
