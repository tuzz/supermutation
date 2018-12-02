use super::*;

type Subject = Interface;

mod ask_for_symbols {
    use super::*;

    #[test]
    #[cfg(not(feature = "four_symbols"))]
    fn it_returns_five_symbols_in_the_test_environment() {
        assert_eq!(Subject::ask_for_symbols(), 5);
    }

    #[test]
    #[cfg(feature = "four_symbols")]
    fn it_returns_four_symbols_when_the_compiler_feature_is_set() {
        assert_eq!(Subject::ask_for_symbols(), 4);
    }
}
