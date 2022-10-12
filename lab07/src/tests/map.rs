use super::TestMap;
use crate::Map61B;

struct NoTraits {
    t: i32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct JustOrd {
    t: usize,
}

#[test]
fn sanity_generics() {
    let _ = TestMap::<u8, ()>::new();
    let _ = TestMap::<String, Vec<()>>::new();
    let _ = TestMap::<usize, usize>::new();
    let _ = TestMap::<bool, TestMap<bool, String>>::new();
    let _ = TestMap::<JustOrd, NoTraits>::new();
}

#[test]
// Assumes insert, get, get_mut, len, and containsKey work
fn sanity_clear() {
    const LEN: usize = 455;
    let mut b = TestMap::<String, usize>::new();
    for i in 0..LEN {
        let s = format!("hi{}", i);
        b.insert(s.clone(), i + 1);
        assert!(b.get(&s) == Some(&(i + 1)));
        assert!(b.get_mut(&s) == Some(&mut (i + 1)));
        assert!(b.contains_key(&s));
    }
    assert_eq!(LEN, b.len());
    b.clear();
    assert_eq!(0, b.len());
    for i in 0..LEN {
        let s = format!("hi{}", i);
        assert!(b.get(&s) == None);
        assert!(b.get_mut(&s) == None);
        assert!(!b.contains_key(&s));
    }
}

#[test]
// Assumes insert works
fn sanity_contains_key() {
    let s = "water_you_doing_here".to_string();
    let mut b = TestMap::<String, i32>::new();
    assert!(!b.contains_key(&s));
    b.insert(s.clone(), 123);
    assert!(b.contains_key(&s));
}

#[test]
// Assumes len works
fn sanity_put_get_and_contains() {
    let s1 = "starChild".to_string();
    let v1 = NoTraits { t: 12345 };
    let s2 = "KISS".to_string();
    let v2 = NoTraits { t: 56789 };
    let mut b = TestMap::<String, NoTraits>::new();

    assert!(!b.contains_key(&s1));
    assert!(b.get(&s1).is_none());
    assert!(b.get_mut(&s1).is_none());
    assert!(b.insert(s1.clone(), v1).is_none());
    assert!(b.contains_key(&s1));
    assert_eq!(b.get(&s1).unwrap().t, 12345);
    b.get_mut(&s1).unwrap().t = 111;
    assert_eq!(b.get(&s1).unwrap().t, 111);

    assert!(b.insert(s2.clone(), v2).is_none());
    assert!(b.contains_key(&s1));
    assert!(b.contains_key(&s2));
    assert_eq!(b.get(&s1).unwrap().t, 111);
    assert_eq!(b.len(), 2);
}

#[test]
// Assumes insert works
fn sanity_len_test() {
    let mut b = TestMap::<String, i32>::new();
    assert_eq!(0, b.len());
    b.insert("hi".to_string(), 0);
    assert_eq!(1, b.len());
    for i in 0..455 {
        b.insert(format!("hi{}", i), 0);
        assert_eq!(i + 2, b.len());
    }
}

#[test]
fn contains_none_test() {
    let s = "hi".to_string();
    let mut b = TestMap::<String, Option<i32>>::new();
    b.insert(s.clone(), None);
    assert!(b.contains_key(&s));
}

#[test]
fn tree_test() {
    let (k1, mut v1) = ("d".to_string(), "parmesan".to_string());
    let (k2, mut v2) = ("a".to_string(), "mozzarella".to_string());
    let (k3, mut v3) = ("c".to_string(), "swiss".to_string());
    let (k4, mut v4) = ("b".to_string(), "pepper jack".to_string());
    let (k5, mut v5) = ("e".to_string(), "gouda".to_string());
    let mut new_v4 = "provolone".to_string();

    let mut b = TestMap::<String, String>::new();
    assert!(b.insert(k1.clone(), v1.clone()).is_none());
    assert!(b.insert(k2.clone(), v2.clone()).is_none());
    assert!(b.insert(k3.clone(), v3.clone()).is_none());
    assert!(b.insert(k4.clone(), v4.clone()).is_none());
    assert!(b.insert(k5.clone(), v5.clone()).is_none());
    assert_eq!(b.len(), 5);
    assert_eq!(b.get(&k1), Some(&v1));
    assert_eq!(b.get(&k2), Some(&v2));
    assert_eq!(b.get(&k3), Some(&v3));
    assert_eq!(b.get(&k4), Some(&v4));
    assert_eq!(b.get(&k5), Some(&v5));
    assert_eq!(b.get_mut(&k1), Some(&mut v1));
    assert_eq!(b.get_mut(&k2), Some(&mut v2));
    assert_eq!(b.get_mut(&k3), Some(&mut v3));
    assert_eq!(b.get_mut(&k4), Some(&mut v4));
    assert_eq!(b.get_mut(&k5), Some(&mut v5));
    assert_eq!(b.insert(k4.clone(), new_v4.clone()), Some(v4.clone()));
    assert_eq!(b.len(), 5);
    assert_eq!(b.get(&k4), Some(&new_v4));
    assert_eq!(b.get_mut(&k4), Some(&mut new_v4));
}

#[test]
fn functionality_test() {
    let mut dictionary = TestMap::<&'static str, &'static str>::new();
    let mut student_ids = TestMap::<&'static str, usize>::new();

    assert_eq!(dictionary.len(), 0);

    // can put objects in ductionary and get them
    assert!(dictionary.insert("hello", "world").is_none());
    assert!(dictionary.contains_key(&"hello"));
    assert_eq!(dictionary.get(&"hello"), Some(&"world"));
    assert_eq!(dictionary.get_mut(&"hello"), Some(&mut "world"));
    assert_eq!(dictionary.len(), 1);

    // putting with existing key updates the value
    assert_eq!(dictionary.insert("hello", "kevin"), Some("world"));
    assert_eq!(dictionary.len(), 1);
    assert!(dictionary.contains_key(&"hello"));
    assert_eq!(dictionary.get(&"hello"), Some(&"kevin"));
    assert_eq!(dictionary.get_mut(&"hello"), Some(&mut "kevin"));

    // putting key multiple times does not affect behavior
    assert!(student_ids.insert("sarah", 12345).is_none());
    assert_eq!(student_ids.len(), 1);
    assert_eq!(student_ids.get(&"sarah"), Some(&12345));
    assert_eq!(student_ids.get_mut(&"sarah"), Some(&mut 12345));
    assert!(student_ids.insert("alan", 345).is_none());
    for _ in 0..3 {
        assert_eq!(student_ids.len(), 2);
        assert_eq!(student_ids.get(&"sarah"), Some(&12345));
        assert_eq!(student_ids.get_mut(&"sarah"), Some(&mut 12345));
        assert_eq!(student_ids.get(&"alan"), Some(&345));
        assert_eq!(student_ids.get_mut(&"alan"), Some(&mut 345));
        assert_eq!(student_ids.insert("alan", 345), Some(345));
    }

    // handle values being the same
    assert_eq!(student_ids.get(&"alan"), Some(&345));
    assert_eq!(student_ids.get_mut(&"alan"), Some(&mut 345));
    assert!(student_ids.insert("evil alan", 345).is_none());
    assert_eq!(student_ids.get(&"alan"), Some(&345));
    assert_eq!(student_ids.get_mut(&"alan"), Some(&mut 345));
    assert_eq!(student_ids.get(&"evil alan"), Some(&345));
    assert_eq!(student_ids.get_mut(&"evil alan"), Some(&mut 345));
}
