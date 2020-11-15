use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::remove_kdigits(String::from("1432219"), 3),
        String::from("1219")
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::remove_kdigits(String::from("10200"), 1),
        String::from("200")
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::remove_kdigits(String::from("15"), 2),
        String::from("0")
    );
}
