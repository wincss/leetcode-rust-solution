use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
    output.sort();
    assert_eq!(output, vec![2]);
}

#[test]
fn case_2() {
    let mut output = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
    output.sort();
    assert_eq!(output, vec![4, 9]);
}

#[test]
fn case_3() {
    let mut output = Solution::intersection(vec![1, 2, 2, 1], vec![8]);
    output.sort();
    assert_eq!(output.len(), 0);
}

#[test]
fn case_4() {
    let mut output = Solution::intersection(vec![1], vec![]);
    output.sort();
    assert_eq!(output.len(), 0);
}
