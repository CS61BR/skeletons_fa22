/// tests for add_constant
mod add_constant {
    use crate::int_list_debugging::{int_list::IntList, int_list_exercises::add_constant};

    #[test]
    fn test_add_one() {
        let mut lst = IntList::from(vec![1, 2, 3, 4, 5]);
        add_constant(&mut lst, 1);
        assert_eq!("[2 -> 3 -> 4 -> 5 -> 6 -> Empty]", lst.to_string());
    }

    #[test]
    fn test_add_two() {
        let mut lst = IntList::from(vec![1, 2, 3, 4, 5]);
        add_constant(&mut lst, 2);
        assert_eq!("[3 -> 4 -> 5 -> 6 -> 7 -> Empty]", lst.to_string());
    }

    #[test]
    fn test_larger_list() {
        let mut lst = IntList::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        add_constant(&mut lst, 10);
        assert_eq!(
            "[11 -> 12 -> 13 -> 14 -> 15 -> 16 -> 17 -> 18 -> 19 -> Empty]",
            lst.to_string()
        );
    }
}

/// tests for set_to_zero_if_max_fel
mod set_to_zero_if_max_fel {
    use crate::int_list_debugging::{
        int_list::IntList, int_list_exercises::set_to_zero_if_max_fel,
    };

    #[test]
    fn test_1() {
        let mut lst = IntList::from(vec![1, 22, 15]);
        set_to_zero_if_max_fel(&mut lst);
        assert_eq!("[0 -> 0 -> 15 -> Empty]", lst.to_string());
    }

    #[test]
    fn test_2() {
        let mut lst = IntList::from(vec![55, 22, 45, 44, 5]);
        set_to_zero_if_max_fel(&mut lst);
        assert_eq!("[0 -> 22 -> 45 -> 0 -> 0 -> Empty]", lst.to_string());
    }

    #[test]
    fn test_3() {
        let mut lst = IntList::from(vec![5, 535, 35, 11, 10, 0]);
        set_to_zero_if_max_fel(&mut lst);
        assert_eq!("[0 -> 0 -> 35 -> 0 -> 10 -> 0 -> Empty]", lst.to_string());
    }
}

/// tests for square_primes
mod square_primes {
    use crate::int_list_debugging::{int_list::IntList, int_list_exercises::square_primes};

    /// Here is a test for the square_primes method. Try running it.
    /// It passes, but the starter code implementation of square_primes
    /// is broken. Write your own test to try to uncover the bug!
    #[test]
    fn passing() {
        let mut lst = IntList::from(vec![14, 15, 16, 17, 18]);
        let changed = square_primes(&mut lst);
        assert_eq!("[14 -> 15 -> 16 -> 289 -> 18 -> Empty]", lst.to_string());
        assert!(changed);
    }
}
