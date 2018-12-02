#![cfg(feature = "four_symbols")]

use super::*;
use crate::open_set::OpenSet;
use crate::closed_set::ClosedSet;

type Subject = Incremental;

fn subject() -> Subject {
    let open_set = OpenSet::new();
    let closed_set = ClosedSet::new();
    let search = Search::new(open_set, closed_set);
    let heuristic = Heuristic::new();

    Subject::new(heuristic, search)
}

mod shortest_path {
    use super::*;

    #[test]
    fn it_incrementally_finds_the_shortest_path() {
        let mut subject = subject();
        let candidate = Candidate::seed();
        let mut milestones = vec![];

        let distance = subject.shortest_path(candidate, |distance, subgoal, _, _| {
            milestones.push((distance, subgoal));
        });

        // The shortest superpermutation for four symbols is 33, but our
        // representation doesn't count the three characters at the start:
        assert_eq!(distance, Some(30));

        assert_eq!(milestones, &[
           (0, 3),    // The shortest path to the start candidate has length 0.
           (1, 4),
           (2, 5),
           (3, 6),
           (5, 7),
           (6, 8),    // The shortest path to a candidate with 8 bits has length 6.
           (7, 9),
           (8, 10),
           (10, 11),
           (11, 12),
           (12, 13),
           (13, 14),
           (15, 15),
           (16, 16),
           (18, 17),
           (19, 18),
           (20, 19),
           (21, 20),
           (23, 21),
           (24, 22),
           (26, 23),
           (27, 24),
           (28, 25),
           (30, 26),  // The shortest path to a superpermutation has length 30.
        ]);
    }
}
