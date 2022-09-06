// uncomment these tests!

/*
use crate::deques::{linkedlistdeque::LinkedListDeque, Deque, arraydeque::ArrayDeque};

#[test]
fn add_is_empty_size_test() {
    let mut lld = LinkedListDeque::new();
    assert!(lld.is_empty());

    lld.add_first("front");
    assert_eq!(1, lld.len());

    lld.add_last("middle");
    assert_eq!(2, lld.len());

    lld.add_last("back");
    assert_eq!(3, lld.len());

    // assert_eq!(format!("{:?}", lld), "")
    assert_eq!(lld.to_string(), "[front, middle, back]");
}

#[test]
fn add_remove_test() {
    let mut lld = LinkedListDeque::new();
    assert!(lld.is_empty());

    lld.add_first(10);
    assert_eq!(false, lld.is_empty());

    let el = lld.remove_first();
    assert_eq!(el, Some(10));
    assert!(lld.is_empty());

    let el = lld.remove_first();
    assert_eq!(el, None);
    assert!(lld.is_empty());

    let el = lld.remove_first();
    assert_eq!(el, None);
    assert!(lld.is_empty());
}

#[test]
fn remove_empty_test() {
    let mut lld = LinkedListDeque::new();
    lld.add_first(10);

    let el = lld.remove_last();
    assert_eq!(el, Some(10));
    assert!(lld.is_empty());

    let el = lld.remove_first();
    assert_eq!(el, None);
    assert!(lld.is_empty());

    let el = lld.remove_last();
    assert_eq!(el, None);
    assert!(lld.is_empty());

    let el = lld.remove_first();
    assert_eq!(el, None);
    assert!(lld.is_empty());

    assert_eq!(lld.len(), 0);
}

#[test]
fn multiple_param_test() {
    let mut lld = LinkedListDeque::new();
    lld.add_first(10 as i32);
    assert_eq!(lld.remove_last(), Some(10));

    let mut lld = LinkedListDeque::new();
    lld.add_first(10. as f64);
    assert_eq!(lld.remove_last(), Some(10.));

    let mut lld = LinkedListDeque::new();
    lld.add_first(10 as usize);
    assert_eq!(lld.remove_last(), Some(10));

    let mut lld = LinkedListDeque::new();
    lld.add_first(true);
    assert_eq!(lld.remove_last(), Some(true));

    let mut lld = LinkedListDeque::new();
    lld.add_first("cheese");
    assert_eq!(lld.remove_last(), Some("cheese"));

    let mut lld = LinkedListDeque::new();
    lld.add_first("cheese".to_string());
    assert_eq!(lld.remove_last(), Some("cheese".to_string()));
}

#[test]
fn really_empty_test() {
    let mut lld = LinkedListDeque::<i32>::new();
    assert_eq!(lld.remove_first(), None);
    assert_eq!(lld.remove_last(), None);
    assert_eq!(lld.get_first(), None);
    assert_eq!(lld.get_last(), None);
    assert_eq!(lld.get_first_mut(), None);
    assert_eq!(lld.get_last_mut(), None);
}

#[test]
fn big_test() {
    const N: usize   = 1000000;
    const MID: usize =  500000;
    let mut lld = LinkedListDeque::new();
    for i in 0..N {
        lld.add_last(i);
    }
    for i in 0..MID {
        assert_eq!(Some(i), lld.remove_first());
    }
    for i in (MID + 1..N).rev() {
        assert_eq!(Some(i), lld.remove_last());
    }
}

#[test]
fn get_test() {
    let mut lld = LinkedListDeque::new();
    lld.add_first(10);
    lld.add_first(5);

    let b = lld.get_first_mut().unwrap();
    *b = 64;

    let b = lld.get_last_mut().unwrap();
    *b = 65;

    lld.add_last(66);
    assert_eq!(Some(&64), lld.get_first());
    assert_eq!(Some(&66), lld.get_last());
    assert_eq!(lld.to_string(), "[64, 65, 66]");
}
*/
