use super::*;

type Subject = Symmetry;

mod transpositions {
    use super::*;

    #[test]
    fn it_returns_the_transposed_permutations_for_all_possible_string_expansions() {
        // 01 -> { 010 } -> { 10 }
        let result = Subject::transpositions(2);
        assert_eq!(result, [
           [[1, 0]]
        ]);

        // 012 -> { 0120, 0121 } -> { 120, *21 }
        let result = Subject::transpositions(3);
        assert_eq!(result, [
           [[1, 2, 0]],
           [[0, 2, 1]]
        ]);

        // 0123 -> { 01230, 01231, 01232 } -> { 1230, *231, **32 }
        let result = Subject::transpositions(4);
        assert_eq!(result, [
            vec![[1, 2, 3, 0]],
            vec![[0, 2, 3, 1]],
            vec![[0, 1, 3, 2], [1, 0, 3, 2]],
        ]);

        // 01234 -> { 012340, 012341, 012342, 012343 } -> { 12340, *2341, **342, ***43 }
        let result = Subject::transpositions(5);
        assert_eq!(result, [
            vec![[1, 2, 3, 4, 0]],                                    // 0!
            vec![[0, 2, 3, 4, 1]],                                    // 1!
            vec![[0, 1, 3, 4, 2], [1, 0, 3, 4, 2]],                   // 2!

            vec![[0, 1, 2, 4, 3], [0, 2, 1, 4, 3], [1, 0, 2, 4, 3],   // 3!
                 [1, 2, 0, 4, 3], [2, 0, 1, 4, 3], [2, 1, 0, 4, 3]],
        ]);
    }
}

mod mappings {
    use super::*;

    #[test]
    fn it_returns_the_index_mappings_to_transpose_each_permutation() {
        // 012 -> { 0120, 0121 } -> { 120, *21 }
        let result = Subject::mappings(3);
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
