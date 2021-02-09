use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2),
        7
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3),
        3
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::subarrays_with_k_distinct(vec![2, 1, 2, 1, 2], 2),
        10
    );
}
