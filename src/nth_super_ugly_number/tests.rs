use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
}

#[test]
fn case_2() {
    assert_eq!(Solution::nth_super_ugly_number(1, vec![2, 3, 5]), 1);
}
