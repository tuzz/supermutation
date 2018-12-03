#![cfg(not(feature = "four_symbols"))]

use super::*;
use crate::candidate::Candidate;

type Subject = Search;

fn subject(candidate: &Candidate) -> Subject {
    let mut open_set = OpenSet::new();
    let closed_set = ClosedSet::new();

    open_set.seed(candidate.clone());

    Subject::new(open_set, closed_set)
}

mod seed {
    use super::*;

    #[test]
    fn it_adds_the_candidate_to_the_open_set() {
        let open_set = OpenSet::new();
        let closed_set = ClosedSet::new();

        let mut subject = Subject::new(open_set, closed_set);
        let candidate = Candidate::seed();

        subject.seed(candidate);
        assert_eq!(subject.open_set.len(), 1);
    }
}

mod shortest_path {
    use super::*;

    // We need to update the heuristic, otherwise an array index is out of bounds.
    fn shortest_path(subject: &mut Subject, goal: usize) -> Option<usize> {
        let mut heuristic = subject.heuristic.clone();
        let distance = subject.shortest_path(goal);

        heuristic.improve_based_on(distance?);
        subject.update_heuristic(&heuristic);

        distance
    }

    #[test]
    fn it_finds_the_length_of_the_shortest_path_to_the_goal_number_of_bits() {
        let start = Candidate::seed();
        let mut subject = &mut subject(&start);

        let mut goal = start.number_of_bits();            // shortest path:
        assert_eq!(shortest_path(subject, goal), Some(0)); // 01234

        goal += 1;
        assert_eq!(shortest_path(subject, goal), Some(1)); // 012340

        goal += 1;
        assert_eq!(shortest_path(subject, goal), Some(2)); // 0123401

        goal += 1;
        assert_eq!(shortest_path(subject, goal), Some(3)); // 01234012

        goal += 1;
        assert_eq!(shortest_path(subject, goal), Some(4)); // 012340123

        goal += 1;                // not possible in 5 moves
        assert_eq!(shortest_path(subject, goal), Some(6)); // 01234012310
    }

    #[test]
    fn it_reuses_the_open_and_closed_sets_in_between_searches() {
        let start = Candidate::seed();
        let mut subject = &mut subject(&start);

        let mut goal = start.number_of_bits();
        shortest_path(subject, goal);

        assert_eq!(subject.open_set.len(), 4);
        assert_eq!(subject.closed_set.len(), 1);

        goal += 1;
        shortest_path(subject, goal);

        assert_eq!(subject.open_set.len(), 15);
        assert_eq!(subject.closed_set.len(), 5);
    }
}

mod open_set_len {
    use super::*;

    #[test]
    fn it_returns_the_number_of_candidates_in_the_open_set() {
        let start = Candidate::seed();

        let mut subject = subject(&start);
        assert_eq!(subject.open_set_len(), 1);

        subject.shortest_path(4);
        assert_eq!(subject.open_set_len(), 4);
    }
}

mod closed_set_len {
    use super::*;

    #[test]
    fn it_returns_the_number_of_candidates_in_the_closed_set() {
        let start = Candidate::seed();

        let mut subject = subject(&start);
        assert_eq!(subject.closed_set_len(), 0);

        subject.shortest_path(4);
        assert_eq!(subject.closed_set_len(), 1);
    }
}

//mod update_heuristic {
//    use super::*;
//
//    fn subject(f_cost: usize) -> Subject {
//        let candidate = Candidate::seed();
//        let mut open_set = OpenSet::new();
//        let closed_set = ClosedSet::new();
//
//        open_set.add(candidate, f_cost, 0);
//
//        Subject::new(open_set, closed_set)
//    }
//
//    #[test]
//    fn it_sets_the_heuristic_for_the_search() {
//        let mut subject = subject(123);
//
//        let heuristic = Heuristic::new();
//        subject.update_heuristic(&heuristic);
//
//        assert_eq!(subject.heuristic, heuristic);
//    }
//
//    mod when_the_heuristic_changed_previous_values {
//        use super::*;
//
//        #[test]
//        fn it_recalculates_the_open_set_costs() {
//            let mut subject = subject(1);
//            let heuristic = Heuristic { changed_previous_values: true, costs: vec![2] };
//
//            subject.update_heuristic(&heuristic);
//
//            let f_cost = subject.open_set.minimum_f_cost();
//            assert_eq!(subject.open_set_len(), 1);
//            assert_eq!(f_cost, Some(2));
//        }
//    }
//
//    mod when_the_heuristic_has_not_changed_previous_values {
//        use super::*;
//
//        #[test]
//        fn it_does_not_recalculate_the_open_set_costs() {
//            let mut subject = subject(123);
//            let heuristic = Heuristic { changed_previous_values: false, costs: vec![1] };
//
//            subject.update_heuristic(&heuristic);
//
//            let f_cost = subject.open_set.minimum_f_cost();
//            assert_eq!(f_cost, Some(123));
//        }
//    }
//}
