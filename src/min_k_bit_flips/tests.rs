use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 2), -1);
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3),
        3
    );
}
