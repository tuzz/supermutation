use super::*;

type Subject = Util;

mod factorial {
    use super::*;

    #[test]
    fn it_calculates_the_factorial_of_a_number() {
        assert_eq!(Subject::factorial(0), 1);
        assert_eq!(Subject::factorial(1), 1);
        assert_eq!(Subject::factorial(2), 2);
        assert_eq!(Subject::factorial(3), 6);
        assert_eq!(Subject::factorial(4), 24);
        assert_eq!(Subject::factorial(5), 120);
    }
}
