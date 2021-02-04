use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
        12.75
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_max_average(vec![10000, -1, -1, -1], 2),
        4999.5
    );
}
