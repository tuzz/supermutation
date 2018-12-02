use super::*;
use crate::candidate::Candidate;

type Subject = Search;

fn subject(candidate: &Candidate) -> Subject {
    let open_set = OpenSet::seed(candidate.clone());
    let closed_set = ClosedSet::new();

    Subject::new(open_set, closed_set)
}

mod shortest_path {
    use super::*;

    #[test]
    fn it_finds_the_length_of_the_shortest_path_to_the_goal_number_of_bits() {
        let start = Candidate::seed();
        let mut subject = subject(&start);

        let mut goal = start.number_of_bits();            // shortest path:
        assert_eq!(subject.shortest_path(goal), Some(0)); // 01234

        goal += 1;
        assert_eq!(subject.shortest_path(goal), Some(1)); // 012340

        goal += 1;
        assert_eq!(subject.shortest_path(goal), Some(2)); // 0123401

        goal += 1;
        assert_eq!(subject.shortest_path(goal), Some(3)); // 01234012

        goal += 1;
        assert_eq!(subject.shortest_path(goal), Some(4)); // 012340123

        goal += 1;                // not possible in 5 moves
        assert_eq!(subject.shortest_path(goal), Some(6)); // 01234012310
    }

    #[test]
    fn it_reuses_the_open_and_closed_sets_in_between_searches() {
        let start = Candidate::seed();
        let mut subject = subject(&start);

        let mut goal = start.number_of_bits();
        subject.shortest_path(goal);

        assert_eq!(subject.open_set.len(), 4);
        assert_eq!(subject.closed_set.len(), 1);

        goal += 1;
        subject.shortest_path(goal);

        assert_eq!(subject.open_set.len(), 15);
        assert_eq!(subject.closed_set.len(), 5);
    }
}
