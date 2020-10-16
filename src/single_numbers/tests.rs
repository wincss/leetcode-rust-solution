use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::single_numbers(vec![4, 1, 4, 6]);
    output.sort();
    assert_eq!(output, [1, 6]);
}

#[test]
fn case_2() {
    let mut output = Solution::single_numbers(vec![1, 2, 10, 4, 1, 4, 3, 3]);
    output.sort();
    assert_eq!(output, [2, 10]);
}
