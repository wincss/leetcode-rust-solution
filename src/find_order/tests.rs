use crate::*;

#[test]
fn case_1() {
    let output = Solution::find_order(2, vec![vec![1, 0]]);
    assert!(output.len() > 0);
    assert_eq!(output, vec![0, 1]);
}

#[test]
fn case_2() {
    let output = Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]);
    assert!(output.len() > 0);
    assert!(output == vec![0, 1, 2, 3] || output == vec![0, 2, 1, 3]);
}

#[test]
fn case_3() {
    let output = Solution::find_order(4, vec![vec![1, 0], vec![2, 1], vec![1, 3], vec![3, 2]]);
    assert_eq!(output, Vec::<i32>::new());
}
