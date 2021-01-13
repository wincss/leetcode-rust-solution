use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::prefixes_div_by5(vec![0, 1, 1]),
        vec![true, false, false]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::prefixes_div_by5(vec![1, 1, 1]),
        vec![false, false, false]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::prefixes_div_by5(vec![0, 1, 1, 1, 1, 1]),
        vec![true, false, false, false, true, false]
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::prefixes_div_by5(vec![1, 1, 1, 0, 1]),
        vec![false, false, false, false, false]
    );
}
