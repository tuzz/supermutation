#![cfg(not(feature = "four_symbols"))]

use super::*;

type Subject = Symmetry;

mod transpositions {
    use super::*;

    #[test]
    fn it_returns_the_transposed_permutations_for_all_possible_string_expansions() {
        // 01 -> { 010 } -> { 10 }
        let result = Subject::transpositions(2, |_, t| t);
        assert_eq!(result, [
           [[1, 0]]
        ]);

        // 012 -> { 0120, 0121 } -> { 120, *21 }
        let result = Subject::transpositions(3, |_, t| t);
        assert_eq!(result, [
           [[1, 2, 0]],
           [[0, 2, 1]]
        ]);

        // 0123 -> { 01230, 01231, 01232 } -> { 1230, *231, **32 }
        let result = Subject::transpositions(4, |_, t| t);
        assert_eq!(result, [
            vec![[1, 2, 3, 0]],
            vec![[0, 2, 3, 1]],
            vec![[0, 1, 3, 2], [1, 0, 3, 2]],
        ]);

        // 01234 -> { 012340, 012341, 012342, 012343 } -> { 12340, *2341, **342, ***43 }
        let result = Subject::transpositions(5, |_, t| t);
        assert_eq!(result, [
            vec![[1, 2, 3, 4, 0]],                                    // 0!
            vec![[0, 2, 3, 4, 1]],                                    // 1!
            vec![[0, 1, 3, 4, 2], [1, 0, 3, 4, 2]],                   // 2!

            vec![[0, 1, 2, 4, 3], [0, 2, 1, 4, 3], [1, 0, 2, 4, 3],   // 3!
                 [1, 2, 0, 4, 3], [2, 0, 1, 4, 3], [2, 1, 0, 4, 3]],
        ]);
    }
}

mod permutation_mappings {
    use super::*;

    #[test]
    fn it_returns_the_index_mappings_to_transpose_each_permutation() {
        // 012 -> { 0120, 0121 } -> { 120, *21 }
        let result = Subject::permutation_mappings(3, |_, m| m);
        assert_eq!(result, [[[
                    //   For the 120 transposition:
                    //
                    // Index      Mapping   New Index
            4,      //  (0)     012 -> 201     (4)
            5,      //  (1)     021 -> 210     (5)
            1,      //  (2)     102 -> 021     (1)
            0,      //  (3)     120 -> 012     (0)
            3,      //  (4)     201 -> 120     (3)
            2,      //  (5)     210 -> 102     (2)
                    //
                    //
        ]], [[      //   For the 021 transposition:
                    //
                    // Index      Mapping   New Index
            1,      //  (0)     012 -> 021     (1)
            0,      //  (1)     021 -> 012     (0)
            4,      //  (2)     102 -> 201     (4)
            5,      //  (3)     120 -> 210     (5)
            2,      //  (4)     201 -> 102     (2)
            3,      //  (5)     210 -> 120     (3)
                    //
                    //
                    //
        ]]]);
    }
}

mod counter_mappings {
    use super::*;

    // At the end of the bitmap, we store some counter bits which represent the
    // minimum number of symbols that need to be added to the string before new
    // permutations can be seen. For example '0123454' requires at least three
    // more symbols, i.e. 0123454___
    //                              ^-- earliest new permutation
    //
    // The way these bits are updated depends on the symbol we're adding to the
    // string. The mappings are shown in the following diagram. The 'x's show
    // the before and after bitmaps for (0123454) -> (0123454)0
    //
    //
    //  0 1 2  ... 720   723      (index)
    //
    // [ | | | ... |T|x| | ]      (before)
    //              | | | |
    //               \ \ \ \
    //                | | | |
    //                v v v  \
    // [ | | | ... |T|x|x| ]  |   (after)
    //  ^                    /
    //  |                   |
    //   \_________________/
    //
    //
    // The 'T' symbol represents the 'ground truth', which will be set to true
    // by the caller and can also be used to 'discard' values from other cells.
    //
    // For other symbols that are added, e.g. '3', we'd remove that many arrows
    // from the right of the diagram. Here's (0123454) -> (0123454)3
    //
    //
    //  0 1 2  ... 720   723      (index)
    //
    // [ | | | ... |T|x| | ]      (before)
    //              |
    //               \
    //                |
    //                v
    // [ | | | ... |T|x| | ]      (after)
    //
    //
    // As you can see, the only way to add a new permutation to the left hand
    // side of the array is by having all of the counter bits set and seeing a
    // '0' symbol, so that none of the arrows are removed. For example, the
    // transition (012345) -> (012345)0 would set a new permutation.

    #[test]
    fn it_returns_mappings_to_update_the_counter_bits_at_the_end_of_the_bitmap() {
        let tru = 120; // ground truth
        let result = Subject::counter_mappings(5);

        assert_eq!(result, &[
            vec![121, 122,   0],   // symbol 0
            vec![121, 122, tru],   // symbol 1
            vec![121, tru, tru],   // symbol 2
            vec![tru, tru, tru],   // symbol 3
        ]);


        let tru = 720; // ground truth
        let result = Subject::counter_mappings(6);

        assert_eq!(result, &[
            vec![721, 722, 723,   0],   // symbol 0
            vec![721, 722, 723, tru],   // symbol 1
            vec![721, 722, tru, tru],   // symbol 2
            vec![721, tru, tru, tru],   // symbol 3
            vec![tru, tru, tru, tru],   // symbol 4
        ]);
    }
}

mod mapping {
    use super::*;

    // The following examples consider the **32 transpositions: { 0132, 1032}

    fn first() -> Vec<u32> {
        vec![1,0,4,5,2,3,7,6,10,11,8,9,18,19,20,21,22,23,12,13,14,15,16,17,  24,24]
    }                                                                  //     ^
                                                                       // counter bits
    fn second() -> Vec<u32> {                                          //     v
        vec![7,6,10,11,8,9,1,0,4,5,2,3,20,21,18,19,23,22,14,15,12,13,17,16,  24,24]
    }

    lazy_static! {
        static ref SUBJECT: Subject = Subject::precompute(4);
    }

    fn mapping(slice: &[u32]) -> &Vec<u32> {
        SUBJECT.mapping(2, &Bitmap::of(slice))
    }

    #[test]
    fn it_chooses_the_mapping_resulting_in_the_bitset_with_leftmost_bits_set() {
        assert_eq!(mapping(&[0]), &first());
        assert_eq!(mapping(&[1]), &first());
        assert_eq!(mapping(&[6]), &second());
        assert_eq!(mapping(&[7]), &second());

        assert_eq!(mapping(&[1, 7]), &first());
        assert_eq!(mapping(&[0, 7]), &second());
        assert_eq!(mapping(&[0, 6]), &first());
        assert_eq!(mapping(&[1, 6]), &first());

        assert_eq!(mapping(&[4, 10, 17]), &second());
        assert_eq!(mapping(&[4, 10, 16, 17]), &first());

        assert_eq!(mapping(&[]), &first());
    }
}
