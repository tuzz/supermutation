use super::*;

type Subject = Candidate;

mod seed {
    use super::*;

    #[test]
    fn it_sets_the_first_bit_in_the_bitmap() {
        let subject = Subject::seed();

        assert_eq!(subject.bitmap.contains(0), true);
        assert_eq!(subject.bitmap.contains(1), false);
    }

    #[test]
    fn it_sets_the_last_n_minus_two_bits() {
        let subject = Subject::seed();

        assert_eq!(subject.bitmap.contains(119), false);
        assert_eq!(subject.bitmap.contains(120), true);
        assert_eq!(subject.bitmap.contains(121), true);
        assert_eq!(subject.bitmap.contains(122), true);
        assert_eq!(subject.bitmap.contains(123), true);
        assert_eq!(subject.bitmap.contains(124), false);
    }
}
