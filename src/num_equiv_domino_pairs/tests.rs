use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
        1
    );
}
