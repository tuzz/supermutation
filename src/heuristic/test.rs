use super::*;

type Subject = Heuristic;

mod new {
    use super::*;

    #[test]
    fn it_sets_the_starting_bits_to_n_minus_one() {
        let subject = Subject::new();
        assert_eq!(subject.starting_bits, 4);
    }

    #[test]
    fn it_sets_the_starting_lower_bounds_based_on_the_starting_bits(){
        let subject = Subject::new();

        // The candidate starts with four bits, so if the search goes in the
        // opposite direction down to three or two bits we know that at a
        // minimum it needs to travel that many units of distance back again.
        assert_eq!(subject.lower_bounds, &[4, 3, 2, 1, 0]);
                                     //    ^  ^        ^----- 4th index
                                     //    |  |
                                     //  These aren't actually possible because
                                     //  bitmaps always have at least two bits.
    }

    #[test]
    fn it_knows_that_the_shortest_path_to_add_zero_bits_has_zero_length() {
        let subject = Subject::new();
        assert_eq!(subject.distances, &[0]);
    }

    #[test]
    fn it_doesnt_invalidate_a_previous_heuristic() {
        let subject = Subject::new();
        assert_eq!(subject.invalidated, false);
    }
}

mod cost {
    use super::*;

    #[test]
    fn it_returns_the_lower_bound_for_the_given_number_of_bits() {
        let subject = Subject::new();

        assert_eq!(subject.cost(4), 0);
        assert_eq!(subject.cost(3), 1);
        assert_eq!(subject.cost(2), 2);
    }
}

mod improve_based_on {
    use super::*;

    #[test]
    fn it_keeps_track_of_the_shortest_path_distances() {
        let mut subject = Subject::new();

        subject.improve_based_on(1);
        assert_eq!(subject.distances, &[0, 1]);

        subject.improve_based_on(3);
        assert_eq!(subject.distances, &[0, 1, 3]);
    }

    #[test]
    fn it() {
        let mut subject = Subject::new();

        subject.improve_based_on(1);
//        assert_eq!(subject.lower_bounds, &[]);

        subject.improve_based_on(2);
        assert_eq!(subject.lower_bounds, &[]);

        subject.improve_based_on(3);
        assert_eq!(subject.lower_bounds, &[]);
    }
}
