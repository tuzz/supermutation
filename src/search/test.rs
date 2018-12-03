#![cfg(not(feature = "four_symbols"))]

use super::*;
use crate::candidate::Candidate;

type Subject = Search;

fn subject(candidate: &Candidate) -> Subject {
    let open_set = OpenSet::new();
    let closed_set = ClosedSet::new();
    let mut subject = Subject::new(open_set, closed_set);

    // Use a simplified heuristic, otherwise these tests become too coupled.
    let heuristic = Heuristic { lower_bounds: vec![5, 4, 3, 2, 1, 0, 0, 0, 0, 0] };

    subject.heuristic = heuristic;
    subject.seed(candidate.clone());
    subject
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

    #[test]
    fn it_finds_the_length_of_the_shortest_path_to_the_goal_number_of_bits() {
        let start = Candidate::seed();
        let mut subject = &mut subject(&start);

        let mut goal = start.number_of_bits() + 1;         // shortest path:
        assert_eq!(subject.shortest_path(goal), Some(1));  // 012340

        goal += 1;
        assert_eq!(subject.shortest_path(goal), Some(2));  // 0123401

        goal += 1;
        assert_eq!(subject.shortest_path(goal), Some(3));  // 01234012

        goal += 1;
        assert_eq!(subject.shortest_path(goal), Some(4));  // 012340123

        goal += 1;                // not possible in 5 moves
        assert_eq!(subject.shortest_path(goal), Some(6));  // 01234012310
    }

    #[test]
    fn it_reuses_the_open_and_closed_sets_in_between_searches() {
        let start = Candidate::seed();
        let mut subject = &mut subject(&start);

        let mut goal = start.number_of_bits() + 1;
        subject.shortest_path(goal);

        assert_eq!(subject.open_set.len(), 4);
        assert_eq!(subject.closed_set.len(), 1);

        goal += 1;
        subject.shortest_path(goal);

        assert_eq!(subject.open_set.len(), 7);
        assert_eq!(subject.closed_set.len(), 2);
    }

    #[test]
    fn it_calculates_the_correct_costs_for_the_expanded_candidates() {
        let start = Candidate::seed();
        let subject = &mut subject(&start);

        let mut goal = start.number_of_bits() + 1;
        subject.shortest_path(goal);
                                                    // b  f  g  h
        assert_eq!(next_bits_and_costs(subject), Some((5, 1, 1, 0)));
        assert_eq!(next_bits_and_costs(subject), Some((4, 2, 1, 1)));
        assert_eq!(next_bits_and_costs(subject), Some((3, 3, 1, 2)));
        assert_eq!(next_bits_and_costs(subject), Some((2, 4, 1, 3)));
        assert_eq!(next_bits_and_costs(subject), None);
    }

    fn next_bits_and_costs(subject: &mut Subject) -> Option<(usize, usize, usize, usize)> {
        let f_cost = subject.open_set.minimum_f_cost()?;
        let (candidate, g_cost) = subject.open_set.next()?;
        let h_cost = f_cost - g_cost;

        Some((candidate.number_of_bits(), f_cost, g_cost, h_cost))
    }
}

mod open_set_len {
    use super::*;

    #[test]
    fn it_returns_the_number_of_candidates_in_the_open_set() {
        let start = Candidate::seed();
        let goal = start.number_of_bits() + 1;

        let subject = &mut subject(&start);
        assert_eq!(subject.open_set_len(), 1);

        subject.shortest_path(goal);
        assert_eq!(subject.open_set_len(), 4);
    }
}

mod closed_set_len {
    use super::*;

    #[test]
    fn it_returns_the_number_of_candidates_in_the_closed_set() {
        let start = Candidate::seed();
        let goal = start.number_of_bits() + 1;

        let subject = &mut subject(&start);
        assert_eq!(subject.closed_set_len(), 0);

        subject.shortest_path(goal);
        assert_eq!(subject.closed_set_len(), 1);
    }
}

mod update_heuristic {
    use super::*;

    fn new_heuristic() -> Heuristic {
        Heuristic { lower_bounds: vec![0, 0, 0, 0, 999, 0, 0, 0, 0, 0] }
    }                                           //  ^ This was 1 previously.

    #[test]
    fn it_sets_the_new_heuristic_for_the_search() {
        let start = Candidate::seed();
        let mut subject = subject(&start);

        subject.update_heuristic(&new_heuristic());

        assert_eq!(subject.heuristic, new_heuristic());
    }

    #[test]
    fn it_recalculates_the_open_set_costs() {
        let start = Candidate::seed();
        let mut subject = subject(&start);

        subject.update_heuristic(&new_heuristic());
        let f_cost = subject.open_set.minimum_f_cost();

        assert_eq!(subject.open_set_len(), 1);
        assert_eq!(f_cost, Some(999));
    }

    #[test]
    fn it_does_not_change_the_g_costs_for_nested_buckets() {
        let start = Candidate::seed();

        let mut before = subject(&start);
        let (_, g_cost_before) = before.open_set.next().unwrap();

        let mut after = subject(&start);
        after.update_heuristic(&new_heuristic());
        let (_, g_cost_after) = after.open_set.next().unwrap();

        assert_eq!(g_cost_before, g_cost_after);

        // TODO: move to open set tests?
    }

    #[test]
    fn it_does_not_reorder_candidates_in_buckets_with_the_same_g_cost() {
        let start = Candidate::seed();
        let goal = start.number_of_bits() + 5;

        let mut subject = subject(&start);
        subject.shortest_path(goal);

        // TODO: move to open set tests?
    }

    #[test]
    fn it_appends_buckets() {
        // TODO: move to open set tests?
    }
}
