use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 1), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);
}
