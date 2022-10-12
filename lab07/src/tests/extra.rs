use super::TestMap;
use crate::Map61B;

macro_rules! strings {
    [$($s: expr),*] => {
        vec![$(($s).to_string()),*]
    };
}

#[test]
fn iter_test() {
    let mut b = TestMap::<String, String>::new();
    let keys = strings!["a", "b", "c", "d", "e"];
    let values = strings!["e", "f", "g", "h", "i"];

    for (k, v) in keys.iter().zip(values.iter()) {
        b.insert(k.clone(), v.clone());
    }

    let mut bvec: Vec<(String, String)> = b.into_iter().collect();
    bvec.sort();
    assert_eq!(bvec.len(), keys.len());
    for i in 0..keys.len() {
        assert_eq!(bvec[i], (keys[i].clone(), values[i].clone()));
    }
}

#[test]
fn test_remove_root() {
    let mut b = TestMap::<String, String>::new();
    let keys = strings!["c", "b", "a", "d", "e"];
    for k in &keys {
        b.insert(k.clone(), k.clone());
    }
    assert_eq!(b.remove(&keys[0]), Some(keys[0].clone()));
    assert_eq!(b.get(&keys[0]), None);
    for k in keys.iter().skip(1) {
        assert_eq!(b.get(k), Some(k));
    }
}

#[test]
fn test_remove_three_cases() {
    let mut b = TestMap::<String, String>::new();
    let keys = strings!["c", "b", "a", "d", "e"];
    for k in &keys {
        b.insert(k.clone(), k.clone());
    }
    assert_eq!(b.remove(&keys[4]), Some(keys[4].clone())); // a b c d
    assert_eq!(b.get(&keys[0]), Some(&keys[0]));
    assert_eq!(b.get(&keys[1]), Some(&keys[1]));
    assert_eq!(b.get(&keys[2]), Some(&keys[2]));
    assert_eq!(b.get(&keys[3]), Some(&keys[3]));
    assert_eq!(b.remove(&keys[0]), Some(keys[0].clone())); // a b d
    assert_eq!(b.get(&keys[1]), Some(&keys[1]));
    assert_eq!(b.get(&keys[2]), Some(&keys[2]));
    assert_eq!(b.get(&keys[3]), Some(&keys[3]));
    assert_eq!(b.insert("f".to_string(), "f".to_string()), None); // a b d f
    assert_eq!(b.remove(&keys[3]), Some(keys[3].clone())); // a b f
    assert_eq!(b.get(&keys[1]), Some(&keys[1]));
    assert_eq!(b.get(&keys[2]), Some(&keys[2]));
    assert_eq!(b.get(&"f".to_string()), Some(&"f".to_string()));
}

#[test]
fn test_remove_root_edge() {
    let mut right = TestMap::<char, char>::new();
    right.insert('A', 'A');
    right.insert('B', 'B');
    assert_eq!(right.remove(&'A'), Some('A'));
    for c in 'C'..'M' {
        right.insert(c, c);
    }
    assert_eq!(right.insert('A', 'a'), None);
    assert_eq!(right.remove(&'D'), Some('D'));
    assert_eq!(right.remove(&'G'), Some('G'));
    assert_eq!(right.remove(&'A'), Some('a'));
    assert_eq!(right.len(), 9);

    let mut left = TestMap::<char, char>::new();
    left.insert('B', 'B');
    left.insert('A', 'A');
    assert_eq!(left.remove(&'B'), Some('B'));
    assert_eq!(left.len(), 1);
    assert_eq!(left.get(&'B'), None);

    let mut no_child = TestMap::<char, char>::new();
    no_child.insert('Z', '$');
    assert_eq!(no_child.remove(&'Z'), Some('$'));
    assert_eq!(no_child.len(), 0);
    assert_eq!(no_child.get(&'Z'), None);
}
