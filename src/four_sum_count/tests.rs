use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
}
