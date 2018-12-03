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
