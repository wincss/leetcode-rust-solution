use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::balanced_string_split(s!("RLRRLLRLRL")), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::balanced_string_split(s!("RLLLLRRRLR")), 3);
}

#[test]
fn case_3() {
    assert_eq!(Solution::balanced_string_split(s!("LLLLRRRR")), 1);
}

#[test]
fn case_4() {
    assert_eq!(Solution::balanced_string_split(s!("RLRRRLLRLL")), 2);
}
