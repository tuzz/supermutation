#![cfg(feature = "four_symbols")]

use super::*;
use crate::open_set::OpenSet;
use crate::closed_set::ClosedSet;

type Subject = Incremental;

fn subject() -> Subject {
    let open_set = OpenSet::new();
    let closed_set = ClosedSet::new();
    let search = Search::new(open_set, closed_set);
    let heuristic = Heuristic::seed();

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
        // representation doesn't count the four characters at the start:
        assert_eq!(distance, Some(29));

        assert_eq!(milestones, &[
           (1, 2),    // The shortest path to 2 perms has length 1.
           (2, 3),
           (3, 4),
           (5, 5),
           (6, 6),
           (7, 7),
           (8, 8),
           (10, 9),
           (11, 10),
           (12, 11),
           (13, 12),
           (16, 13),  // The shortest path to 13 perms has length 16.
           (17, 14),
           (18, 15),
           (19, 16),
           (21, 17),
           (22, 18),
           (23, 19),
           (24, 20),
           (26, 21),
           (27, 22),
           (28, 23),
           (29, 24),  // The shortest path to a superpermutation has length 29.
        ]);
    }

    #[test]
    fn it_sets_all_bits_in_the_candidates_bitmap_when_a_superpermutation_is_reached() {
        let candidate = Candidate::seed();   // 0123
        let candidate = candidate.expand(0); // 01230
        let candidate = candidate.expand(0); // 012301
        let candidate = candidate.expand(0); // 0123012
        let candidate = candidate.expand(1); // 01230120
        let candidate = candidate.expand(0); // 012301203
        let candidate = candidate.expand(0); // 0123012031
        let candidate = candidate.expand(0); // 01230120312
        let candidate = candidate.expand(0); // 012301203120
        let candidate = candidate.expand(1); // 0123012031201
        let candidate = candidate.expand(0); // 01230120312013
        let candidate = candidate.expand(0); // 012301203120132
        let candidate = candidate.expand(0); // 0123012031201320
        let candidate = candidate.expand(0); // 01230120312013201
        let candidate = candidate.expand(2); // 012301203120132010
        let candidate = candidate.expand(1); // 0123012031201320102
        let candidate = candidate.expand(0); // 01230120312013201023
        let candidate = candidate.expand(0); // 012301203120132010231
        let candidate = candidate.expand(0); // 0123012031201320102310
        let candidate = candidate.expand(0); // 01230120312013201023102
        let candidate = candidate.expand(1); // 012301203120132010231021
        let candidate = candidate.expand(0); // 0123012031201320102310213
        let candidate = candidate.expand(0); // 01230120312013201023102130
        let candidate = candidate.expand(0); // 012301203120132010231021302
        let candidate = candidate.expand(0); // 0123012031201320102310213021
        let candidate = candidate.expand(1); // 01230120312013201023102130210
        let candidate = candidate.expand(0); // 012301203120132010231021302103
        let candidate = candidate.expand(0); // 0123012031201320102310213021032
        let candidate = candidate.expand(0); // 01230120312013201023102130210321
        let candidate = candidate.expand(0); // 012301203120132010231021302103210

        assert_eq!(candidate.number_of_bits(), 26);
        assert_eq!(candidate.number_of_permutations(), 24);
    }
}
