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

mod mapping {
    use super::*;

    // The following examples consider the **32 transpositions: { 0132, 1032}

    fn first() -> Vec<u8> {
        vec![1,0,4,5,2,3,7,6,10,11,8,9,18,19,20,21,22,23,12,13,14,15,16,17]
    }

    fn second() -> Vec<u8> {
        vec![7,6,10,11,8,9,1,0,4,5,2,3,20,21,18,19,23,22,14,15,12,13,17,16]
    }

    lazy_static! {
        static ref SUBJECT: Subject = Subject::precompute(4);
    }

    fn mapping(slice: &[u32]) -> &Vec<u8> {
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
