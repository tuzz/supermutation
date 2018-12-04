#![cfg(not(feature = "four_symbols"))]

use super::*;

type Subject = Heuristic;

mod seed {
    use super::*;

    #[test]
    fn it_sets_the_starting_bits_to_n_minus_one() {
        let subject = Subject::seed();
        assert_eq!(subject.starting_bits, 4);
    }

    #[test]
    fn it_sets_the_starting_lower_bounds_based_on_the_starting_bits(){
        let subject = Subject::seed();

        // The first goal is five bits and the candidate starts with four so if
        // the search goes in the opposite direction we know that at a minimum
        // it needs to travel that many units of distance back again.
        assert_eq!(subject.lower_bounds, &[5, 4, 3, 2, 1, 0]);
                                     //    ^  ^        ^-- We start at least one
                                     //    |  |            unit from the goal.
                                     //    |  |
                                     //  These aren't actually possible because
                                     //  bitmaps always have at least two bits.
    }

    #[test]
    fn it_knows_that_the_shortest_path_to_add_zero_bits_has_zero_length() {
        let subject = Subject::seed();
        assert_eq!(subject.distances, &[0]);
    }
}

mod cost {
    use super::*;

    #[test]
    fn it_returns_the_lower_bound_for_the_given_number_of_bits() {
        let subject = Subject::seed();

        assert_eq!(subject.cost(4), 1);
        assert_eq!(subject.cost(3), 2);
        assert_eq!(subject.cost(2), 3);
    }
}

mod first_goal {
    use super::*;

    #[test]
    fn it_returns_the_number_of_bits_in_the_first_subgoal() {
        let mut subject = Subject::seed();
        assert_eq!(subject.first_goal(), 5);

        subject.improve_based_on(1);
        assert_eq!(subject.first_goal(), 5);

        subject.improve_based_on(2);
        assert_eq!(subject.first_goal(), 5);
    }
}

mod next_goal {
    use super::*;

    #[test]
    fn it_returns_the_number_of_bits_in_the_next_subgoal() {
        let mut subject = Subject::seed();
        assert_eq!(subject.next_goal(), 5);

        subject.improve_based_on(1);
        assert_eq!(subject.next_goal(), 6);

        subject.improve_based_on(2);
        assert_eq!(subject.next_goal(), 7);
    }
}

mod improve_based_on {
    use super::*;

    #[test]
    fn it_adds_the_new_distances_to_its_shortest_path_distances() {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);
        assert_eq!(subject.distances, &[0, 1]);

        subject.improve_based_on(2);
        assert_eq!(subject.distances, &[0, 1, 2]);

        subject.improve_based_on(4);
        assert_eq!(subject.distances, &[0, 1, 2, 4]);
    }

    #[test]
    fn it_adds_a_new_lower_bound_of_zero_for_the_next_subgoal() {
        let mut subject = Subject::seed();
        assert_eq!(subject.lower_bounds.len(), 6);

        subject.improve_based_on(1);
        assert_eq!(subject.lower_bounds.len(), 7);
        assert_eq!(subject.lower_bounds.last().unwrap(), &0);
    }

    #[test]
    fn it_increases_the_previous_lower_bounds() {
        let mut subject = Subject::seed();
        assert_eq!(&subject.lower_bounds[4..], &[1, 0]);

        subject.improve_based_on(1);
        assert_eq!(&subject.lower_bounds[4..], &[2, 1, 0]);

        subject.improve_based_on(2);
        assert_eq!(&subject.lower_bounds[4..], &[3, 2, 1, 0]);
    }

    #[test]
    fn it_updates_the_lower_bounds_taking_into_account_previous_gaps() {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);
        assert_eq!(&subject.lower_bounds[4..], &[2, 1, 0]);

        subject.improve_based_on(3);
        assert_eq!(&subject.lower_bounds[4..], &[4, 3, 1, 0]);

        //   Graph of number of bits vs. distance:
        //
        //          |     o           <-- lower bound for 6 bits
        // distance |    x (5, 3)     <-- needs to travel at least 1 distance
        //          |
        //          |   x (4, 1)      <--   "   "    "    "    "   3 distance
        //          |__x___________   <--   "   "    "    "    "   4 distance
        //             (3, 0)
        //
        //           number of bits
    }

    #[test]
    fn it_sets_higher_lower_bounds_when_there_was_a_previous_gap_in_distance() {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);
        assert_eq!(&subject.lower_bounds[4..], &[2, 1, 0]);

        subject.improve_based_on(3);
        assert_eq!(&subject.lower_bounds[4..], &[4, 3, 1, 0]);

        subject.improve_based_on(4);
        assert_eq!(&subject.lower_bounds[4..], &[6, 5, 3, 2, 0]);

        //   Graph of number of bits vs. distance:
        //
        //          |      o          <-- lower bound for 7 bits is +2 because of 4 to 5 bits
        //          |
        //          |     x (6, 4)    <-- needs to travel at least 2 distance
        // distance |    x (5, 3)     <--   "   "    "    "    "   3 distance
        //          |
        //          |   x (4, 1)      <--   "   "    "    "    "   5 distance
        //          |__x___________   <--   "   "    "    "    "   6 distance
        //             (3, 0)
        //
        //           number of bits
    }

    #[test]
    fn it_sets_the_lower_bound_from_the_biggest_previous_gap_it_finds() {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);
        subject.improve_based_on(3);  // gap of 2
        subject.improve_based_on(4);
        subject.improve_based_on(7);  // gap of 3
        subject.improve_based_on(8);
        subject.improve_based_on(10); // gap of 2
        subject.improve_based_on(11);

        assert_eq!(&subject.lower_bounds[4..], &[14, 13, 11, 10, 7, 6, 4, 3, 0]);

        //   Graph of number of bits vs. distance:
        //
        //          |          o            <-- lower bound for 11 bits is +3
        //          |                           because of 6 to 7 bits
        //          |
        //          |         x (10, 11)    <-- needs to travel at least 3 distance
        //          |        x (9, 10)      <--   "   "    "    "    "   4 distance
        // distance |
        //          |       x (8, 8)        <--   "   "    "    "    "   6 distance
        //          |      x (7, 7)         <--   "   "    "    "    "   7 distance
        //          |
        //          |
        //          |     x (6, 4)          <--   "   "    "    "    "   10 distance
        //          |    x (5, 3)           <--   "   "    "    "    "   11 distance
        //          |
        //          |   x (4, 1)            <--   "   "    "    "    "   13 distance
        //          |__x___________         <--   "   "    "    "    "   14 distance
        //             (3, 0)
        //
        //           number of bits
    }

    #[test]
    fn it_increases_the_lower_bounds_for_candidates_with_fewer_than_the_starting_bits() {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);
        subject.improve_based_on(3);  // gap of 2
        subject.improve_based_on(4);
        subject.improve_based_on(7);  // gap of 3
        subject.improve_based_on(8);
        subject.improve_based_on(10); // gap of 2
        subject.improve_based_on(11);

        // I think this is the right thing to do, but I'm not 100% sure.
        assert_eq!(&subject.lower_bounds, &[21, 18, 17, 15, 14, 13, 11, 10, 7, 6, 4, 3, 0]);
    }
}
