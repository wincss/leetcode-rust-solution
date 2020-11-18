use crate::*;

#[test]
fn case_1() {
    let mut input = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut input);
    assert_eq!(input, vec![1, 3, 12, 0, 0]);
}
