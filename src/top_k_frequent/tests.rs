use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
    output.sort();
    assert_eq!(output, vec![1, 2]);
}

#[test]
fn case_2() {
    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}
