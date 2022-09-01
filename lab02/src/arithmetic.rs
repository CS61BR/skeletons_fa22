pub fn product(a: i32, b: i32) -> i32 {
    a * b
}

pub fn sum(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::{product, sum};

    // Performs a few arbitrary tests to see if the product method is correct
    #[test]
    fn test_product() {
        // assert_eq! will panic if the two arguments are not equal, causing the test to fail
        assert_eq!(product(5, 6), 30);
        assert_eq!(product(5, -6), -30);
        assert_eq!(product(0, 6), 0);
    }

    // Performs a few arbitrary tests to see if the sum method is correct
    #[test]
    fn test_sum() {
        assert_eq!(sum(5, 6), 11);
        assert_eq!(sum(5, -6), -1);
        assert_eq!(sum(0, -6), -6);
        assert_eq!(sum(6, -6), 0);
    }
}
