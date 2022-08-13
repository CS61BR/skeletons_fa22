#[cfg(test)]
mod tests {
    use crate::is_leap_year;

    #[test]
    fn test_400() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2400));
    }

    #[test]
    fn test_leap_year_not_divisible_100() {
        assert!(is_leap_year(2004));
        assert!(is_leap_year(2008));
    }

    #[test]
    fn test_divisible_100_not_400() {
        assert_eq!(is_leap_year(1900), false);
        assert_eq!(is_leap_year(2100), false);
        assert_eq!(is_leap_year(2300), false);
    }

    #[test]
    fn test_divisible_not_4() {
        assert_eq!(is_leap_year(1901), false);
        assert_eq!(is_leap_year(2003), false);
    }
}
