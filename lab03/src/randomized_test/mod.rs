use self::{alistnoresizing::AListNR, buggyalist::BAList};

mod alistnoresizing;
mod buggyalist;

#[test]
fn test_add_3_remove_3() {
    let mut correct: AListNR<i32> = AListNR::new();
    let mut broken: BAList<i32> = BAList::new();

    unimplemented!(); // TODO: fill in this test
}

#[test]
fn randomized_test() {
    let mut correct: AListNR<i32> = AListNR::new();
    let mut broken: BAList<i32> = BAList::new();

    println!("{}", rand::random::<u32>() % 4);

    unimplemented!(); // TODO: fill in this test
}
