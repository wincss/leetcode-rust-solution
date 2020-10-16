use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::missing_two(vec![1]);
    output.sort();
    assert_eq!(output, vec![2, 3]);
}

#[test]
fn case_2() {
    let mut output = Solution::missing_two(vec![2, 3]);
    output.sort();
    assert_eq!(output, vec![1, 4]);
}
