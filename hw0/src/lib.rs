mod solutions;
pub use solutions::*;

/*
Feel free to add your own tests. For now, you can just follow the pattern you
see here. We'll look into the details of unit testing later.

To actually run the tests, just use
cargo test
*/

#[cfg(test)]
mod tests {
    use crate::solutions::*;

    #[test]
    fn is_even_test() {
        assert_eq!(is_even(10), true);
        assert_eq!(is_even(1), false);
        assert_eq!(is_even(99), false);
        // REPLACE THIS WITH MORE TESTS
    }

    #[test]
    fn word_bank_test() {
        let adjectives = vec![
            String::from("big"),
            String::from("cool"),
            String::from("cheesy"),
        ];
        assert_eq!(word_bank(String::from("big"), adjectives.clone()), true);
        assert_eq!(word_bank(String::from("dog"), adjectives.clone()), false);
        // REPLACE THIS WITH MORE TESTS
    }

    // UNCOMMENT EVERYTHING BELOW TO RUN MORE TESTS.
    // #[test]
    // fn max_test() {
    //     assert_eq!(max(vec![1,2,3,4,5]), 5);
    //     assert_eq!(max(vec![0,-5,2,14,10]), 14);
    //     assert_eq!(max(vec![1]), 1);
    //     // REPLACE THIS WITH MORE TESTS
    // }

    // #[test]
    // fn three_sum_test() {
    //     assert_eq!(three_sum(vec![1,2,-3]), true); // 1 + 2 - 3 = 0
    //     assert_eq!(three_sum(vec![-6, 3, 10, 200]), true); // 3 + 3 - 6 = 0
    //     assert_eq!(three_sum(vec![1, -900, 7]), false);
    //     // REPLACE THIS WITH MORE TESTS
    // }
}
