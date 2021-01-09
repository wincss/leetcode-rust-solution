use crate::*;

#[test]
fn case_1() {
    let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
    Solution::duplicate_zeros(&mut arr);
    assert_eq!(arr, vec![1, 0, 0, 2, 3, 0, 0, 4])
}

#[test]
fn case_2() {
    let mut arr = vec![1, 2, 3];
    Solution::duplicate_zeros(&mut arr);
    assert_eq!(arr, vec![1, 2, 3])
}

#[test]
fn case_3() {
    let mut arr = vec![1, 0, 2, 3, 0, 4];
    Solution::duplicate_zeros(&mut arr);
    assert_eq!(arr, vec![1, 0, 0, 2, 3, 0])
}
