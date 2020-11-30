use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
        vec![-1, -1]
    );
}

#[test]
fn case_3() {
    assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 11),
        vec![-1, -1]
    );
}

#[test]
fn case_5() {
    assert_eq!(Solution::search_range(vec![1], 0), vec![-1, -1]);
}
