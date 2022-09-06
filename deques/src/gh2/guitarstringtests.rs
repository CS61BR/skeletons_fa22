use super::{guitarstring::GuitarString, random::Random};



#[test]
fn test_1() {
    let mut gs = GuitarString::new(100, 0.996, 1);
    for _ in 0..200 {
        assert_eq!(gs.advance(), 0.);
    }
    
    gs.pluck(&mut Random::new("cheese"));
    for _ in 0..100 {
        assert_ne!(gs.advance(), 0.);
    }
}

#[test]
fn test_2() {
    let mut gs = GuitarString::new(100, 0.996, 1);
    gs.pluck(&mut Random::new("cheese"));
    let mut prev = 0.;
    for _ in 1..100 {
        let cur = gs.advance();
        assert_ne!(cur, prev, "samples should not be the same");
        prev = cur;
    }
}

#[test]
fn test_3() {
    let a = 0.4545;
    let mut gs = GuitarString::new(200, a, 50);
    // note that period should be 200/50 = 4

    gs.pluck(&mut Random::new("cheese"));
    let s1 = gs.advance();
    let s2 = gs.advance();
    let s3 = gs.advance();
    let s4 = gs.advance();

    let s5 = gs.advance();
    let expected = (s1 + s2) * 0.5 * a;
    assert!((s5 - expected).abs() < 0.001);

    let s6 = gs.advance();
    let expected = (s2 + s3) * 0.5 * a;
    assert!((s6 - expected).abs() < 0.001);

    let s7 = gs.advance();
    let expected = (s3 + s4) * 0.5 * a;
    assert!((s7 - expected).abs() < 0.001);
}
