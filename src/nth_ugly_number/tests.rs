use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::nth_ugly_number(1), 1);
}

#[test]
fn case_2() {
    assert_eq!(Solution::nth_ugly_number(10), 12);
}

#[test]
fn case_3() {
    assert_eq!(Solution::nth_ugly_number(1407), 536870912);
}
