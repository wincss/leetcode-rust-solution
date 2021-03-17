use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::num_distinct(String::from("rabbbit"), String::from("rabbit")),
        3
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::num_distinct(String::from("babgbag"), String::from("bag")),
        5
    );
}
