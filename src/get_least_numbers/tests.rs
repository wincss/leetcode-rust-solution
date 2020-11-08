use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::get_least_numbers(vec![3, 2, 1], 2);
    output.sort();
    assert_eq!(output, vec![1, 2]);
}

#[test]
fn case_2() {
    let mut output = Solution::get_least_numbers(vec![0, 1, 2, 1], 1);
    output.sort();
    assert_eq!(output, vec![0]);
}
