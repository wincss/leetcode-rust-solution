use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::hamming_weight(0b1011), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::hamming_weight(0b10000000), 1);
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::hamming_weight(0b11111111111111111111111111111101),
        31
    );
}
