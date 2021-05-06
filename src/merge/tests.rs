use crate::*;

#[test]
fn case_1() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    Solution::merge_sorted_array(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}

#[test]
fn case_2() {
    let mut nums1 = vec![1];
    let mut nums2 = vec![];
    Solution::merge_sorted_array(&mut nums1, 1, &mut nums2, 0);
    assert_eq!(nums1, vec![1]);
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::merge_intervals(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::merge_intervals(vec![vec![1, 4], vec![4, 5]]),
        vec![vec![1, 5]]
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::merge_intervals(vec![vec![1, 4], vec![2, 3]]),
        vec![vec![1, 4]]
    );
}
