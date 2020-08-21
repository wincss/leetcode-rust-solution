use crate::*;

#[test]
fn case_1() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
}

#[test]
fn case_2() {
    let mut nums = vec![0, 0, 0, 0, 1, 0];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, vec![0, 0, 0, 0, 0, 1]);
}

#[test]
fn case_3() {
    let mut nums = vec![2];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, vec![2]);
}

#[test]
fn case_4() {
    let mut nums = vec![2, 2];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, vec![2, 2]);
}

#[test]
fn case_5() {
    let mut nums = vec![];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, vec![]);
}
