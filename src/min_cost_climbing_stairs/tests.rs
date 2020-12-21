use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}
