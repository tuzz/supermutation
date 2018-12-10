#![cfg(not(feature = "four_symbols"))]

use super::*;

type Subject = Heuristic;

mod seed {
    use super::*;

    #[test]
    fn it_sets_starting_perms_from_the_seed_candidate() {
        let subject = Subject::seed();
        let expected = Candidate::seed().number_of_permutations();

        assert_eq!(subject.starting_perms, expected);
    }

    #[test]
    fn it_sets_the_length_of_max_depth_so_that_the_next_goal_can_be_used_as_an_index() {
        let subject = Subject::seed();
        let expected = subject.next_goal() + 1;

        assert_eq!(subject.max_depths.len(), expected);
    }

    #[test]
    fn it_sets_the_length_of_lower_bounds_so_that_the_next_goal_can_be_used_as_an_index() {
        let subject = Subject::seed();
        let expected = subject.next_goal() + 1;

        assert_eq!(subject.lower_bounds.len(), expected);
    }

    #[test]
    fn it_sets_max_depth_elements_to_zero() {
        let subject = Subject::seed();
        assert_eq!(subject.max_depths, &[0, 0, 0]);
    }

    #[test]
    fn it_sets_lower_bounds_to_descending_distances() {
        let subject = Subject::seed();

        // The first goal is two perms and the candidate starts with one so if
        // the search goes in the opposite direction we know that at a minimum
        // it needs to travel that many units of distance back again.
        assert_eq!(subject.lower_bounds, &[2, 1, 0]);
                                          //  ^-- We start at least one
                                          //      away from the goal.
    }
}

mod cost {
    use super::*;

    //  The following tests use this graph of number of perms vs. distance:
    //    (see the #improve_based_on tests below for more explanation)
    //
    //           |            o (5, 6)
    //           |           /
    //           |          x (4, 5)
    //           |          |
    //           |          x (4, 4)
    //           |         /
    //  distance |        x (3, 3)
    //           |       /
    //           |      x (2, 2)
    //           |      |
    //           |      x (2, 1)
    //           |     /
    //           |____x_(1, 0)____________
    //
    //            number of perms

    fn setup() -> Subject {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);
        subject.improve_based_on(3);
        subject.improve_based_on(4);

        subject
    }

    #[test]
    fn it_returns_the_shortest_distance_for_candidates_on_the_optimal_path() {
        let subject = setup();

        assert_eq!(subject.cost(4, 5), 1);
        assert_eq!(subject.cost(4, 4), 2);
        assert_eq!(subject.cost(3, 3), 3);
        assert_eq!(subject.cost(2, 2), 4);
        assert_eq!(subject.cost(2, 1), 5);
        assert_eq!(subject.cost(1, 0), 6);
    }

    #[test]
    fn it_returns_the_shortest_distance_for_candidates_off_the_optimal_path() {
        let subject = setup();

        assert_eq!(subject.cost(4, 6), 1);
        assert_eq!(subject.cost(4, 7), 1);
        assert_eq!(subject.cost(4, 8), 1);

        assert_eq!(subject.cost(2, 6), 4);
        assert_eq!(subject.cost(2, 7), 4);
        assert_eq!(subject.cost(2, 8), 4);

        assert_eq!(subject.cost(1, 3), 6);
        assert_eq!(subject.cost(1, 4), 6);
        assert_eq!(subject.cost(1, 5), 6);
    }

    #[test]
    fn it_returns_zero_distance_if_already_at_the_goal() {
        let subject = setup();

        assert_eq!(subject.cost(5, 4), 0);
        assert_eq!(subject.cost(5, 5), 0);
        assert_eq!(subject.cost(5, 6), 0);
        assert_eq!(subject.cost(5, 7), 0);
        assert_eq!(subject.cost(5, 8), 0);
    }

    #[test]
    fn it_returns_the_cumulative_distance_for_candidates_with_less_than_starting_perms() {
        let subject = setup();
        let distance_from_start = 6;

        assert_eq!(subject.cost(0, 0), 1 + distance_from_start);
        assert_eq!(subject.cost(0, 1), 1 + distance_from_start);
        assert_eq!(subject.cost(0, 2), 1 + distance_from_start);
        assert_eq!(subject.cost(0, 3), 1 + distance_from_start);
    }
}

mod improve_based_on {
    use super::*;

    #[test]
    fn it_adds_the_new_distance_to_its_shortest_path_distances() {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);
        assert_eq!(subject.distances, &[0, 1]);

        subject.improve_based_on(3);
        assert_eq!(subject.distances, &[0, 1, 3]);
    }

    #[test]
    fn it_adds_a_new_max_depth_for_the_next_goal() {
        let mut subject = Subject::seed();
        let previous = subject.max_depths.len();

        subject.improve_based_on(1);
        assert_eq!(subject.max_depths.len(), previous + 1);
    }

    #[test]
    fn it_sets_the_new_max_depth_to_zero_for_the_next_goal() {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);

        // If we're at the goal, we want #cost to return 0, so set max_depth to
        // 0 for the index of the goal so that no additional distance is added.
        assert_eq!(subject.max_depths.last(), Some(&0));
    }

    #[test]
    fn it_keeps_a_max_depth_of_zero_for_indexes_below_starting_perms() {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);

        // This is a bit of a special case and it suffices to set this to 0 so
        // that #cost returns the same value, regardless of search depth.
        assert_eq!(&subject.max_depths[..1], &[0]);
    }

    #[test]
    fn it_sets_max_depth_to_the_furthest_distance_from_the_start_for_each_number_of_perms() {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);
        assert_eq!(&subject.max_depths[1..], &[0, 1, 0]);

        //   Graph of number of perms vs. distance:
        //
        //
        //           |        o (3, 2)     <-- best lower bound for 3 perms
        //           |       /
        //  distance |      x (2, 1)       <-- max depth for 2 perms is 1
        //           |     /
        //           |____x_(1, 0)__       <-- max depth for 1 perm is 0
        //
        //            number of perms


        subject.improve_based_on(3);
        assert_eq!(&subject.max_depths[1..], &[0, 2, 3, 0]);

        //   Graph of number of perms vs. distance:
        //
        //
        //           |          o (4, 4)   <-- best lower bound for 4 perms
        //           |         /
        //           |        x (3, 3)     <-- max depth for 3 perms is 3
        //           |       /
        //  distance |      x (2, 2)       <-- max depth for 2 perms is 2
        //           |      |
        //           |      x (2, 1)
        //           |     /
        //           |____x_(1, 0)__       <-- max depth for 1 perm is 0
        //
        //            number of perms
    }

    #[test]
    fn it_adds_a_new_lower_bound_for_the_next_goal() {
        let mut subject = Subject::seed();
        let previous = subject.lower_bounds.len();

        subject.improve_based_on(1);
        assert_eq!(subject.lower_bounds.len(), previous + 1);
    }

    #[test]
    fn it_sets_the_new_lower_bound_to_zero_for_the_next_goal() {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);

        // If we're at the goal, we want #cost to return 0.
        assert_eq!(subject.lower_bounds.last(), Some(&0));
    }

    #[test]
    fn it_adds_the_full_shortest_path_distance_for_indexes_below_starting_perms() {
        let mut subject = Subject::seed();
        assert_eq!(&subject.lower_bounds[..1], &[2]);

        subject.improve_based_on(1);

        // This is a bit of a special case. We can reach the goal if we travel
        // back to the starting perms, then take the shortest path to the goal.
        assert_eq!(&subject.lower_bounds[..1], &[3]);

        subject.improve_based_on(3);
        assert_eq!(&subject.lower_bounds[..1], &[5]);
    }

    #[test]
    fn it_sets_lower_bound_to_the_nearest_distance_to_the_end_for_each_number_of_perms() {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);
        assert_eq!(&subject.lower_bounds[1..], &[2, 1, 0]);

        //   Graph of number of perms vs. distance:
        //
        //
        //           |        o (3, 2)     <-- 3 perms is the goal
        //           |       /
        //  distance |      x (2, 1)       <-- 2 perms is 1 away from the goal
        //           |     /
        //           |____x_(1, 0)__       <-- 1 perm is 2 away from the goal
        //
        //            number of perms


        subject.improve_based_on(3);
        assert_eq!(&subject.lower_bounds[1..], &[4, 2, 1, 0]);

        //   Graph of number of perms vs. distance:
        //
        //
        //           |          o (4, 4)   <-- 4 perms is the goal
        //           |         /
        //           |        x (3, 3)     <-- 3 perms is 1 away from the goal
        //           |       /
        //  distance |      x (2, 2)       <-- 2 perms is 2 away from the goal
        //           |      |
        //           |      x (2, 1)
        //           |     /
        //           |____x_(1, 0)__       <-- 1 perm is 4 away from the goal
        //
        //            number of perms
    }

    #[test]
    fn it_increases_the_lower_bound_of_the_next_goal_based_on_previous_knowledge() {
        let mut subject = Subject::seed();

        subject.improve_based_on(1);
        subject.improve_based_on(3);
        subject.improve_based_on(4);

        //   Graph of number of perms vs. distance:
        //
        //
        //           |            o (5, 6)  <--    best lower bound for 5 perms is
        //           |           /             \   higher, because we know that...
        //           |          x (4, 5)        |
        //           |          |               |- ...we're about to add another
        //           |          x (4, 4)        |     2 perms from here and...
        //           |         /               /
        //           |        x (3, 3)      <--
        //           |       /                  \
        //  distance |      x (2, 2)             |
        //           |      |                    |- ...here we added 2 perms
        //           |      x (2, 1)             |     which required 3 distance
        //           |     /                    /
        //           |____x_(1, 0)__        <--
        //
        //            number of perms

        assert_eq!(subject.max_depths, &[0, 0, 2, 3, 5, 0]);
                                                   //         ^
                                                   // this is higher because
                                                   // of the higher lower bound

        assert_eq!(subject.lower_bounds, &[7, 6, 4, 3, 1, 0]);
                                        // ^  ^  ^  ^
                                        // these are further away because
                                        // of the higher lower bound
    }
}

mod first_goal {
    use super::*;

    #[test]
    fn it_returns_the_number_of_perms_in_the_first_subgoal() {
        let mut subject = Subject::seed();
        assert_eq!(subject.first_goal(), 2);

        subject.improve_based_on(1);
        assert_eq!(subject.first_goal(), 2);

        subject.improve_based_on(2);
        assert_eq!(subject.first_goal(), 2);
    }
}

mod next_goal {
    use super::*;

    #[test]
    fn it_returns_the_number_of_perms_in_the_next_subgoal() {
        let mut subject = Subject::seed();
        assert_eq!(subject.next_goal(), 2);

        subject.improve_based_on(1);
        assert_eq!(subject.next_goal(), 3);

        subject.improve_based_on(2);
        assert_eq!(subject.next_goal(), 4);
    }
}
