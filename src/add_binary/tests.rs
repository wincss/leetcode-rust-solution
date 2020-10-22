use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::add_binary(String::from("11"), String::from("1")),
        String::from("100")
    )
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::add_binary(String::from("1010"), String::from("1011")),
        String::from("10101")
    )
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::add_binary(String::from("0"), String::from("0")),
        String::from("0")
    )
}
