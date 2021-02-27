use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_monotonic(vec![1, 2, 2, 3]));
    assert!(Solution::is_monotonic(vec![6, 5, 4, 4]));
    assert!(!Solution::is_monotonic(vec![1, 3, 2]));
    assert!(Solution::is_monotonic(vec![1, 2, 4, 5]));
    assert!(Solution::is_monotonic(vec![1, 1, 1]));
}
