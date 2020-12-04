use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
}

#[test]
fn case_3() {
    assert_eq!(Solution::reverse_pairs(vec![-8, -4, 1, 3]), 0);
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::reverse_pairs(vec![
            2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647
        ]),
        0
    );
}
