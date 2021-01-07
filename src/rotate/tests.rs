use crate::*;

#[test]
fn case_1() {
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut arr, 3);
    assert_eq!(arr, vec![5, 6, 7, 1, 2, 3, 4]);
}

#[test]
fn case_2() {
    let mut arr = vec![-1, -100, 3, 99];
    Solution::rotate(&mut arr, 2);
    assert_eq!(arr, vec![3, 99, -1, -100]);
}
