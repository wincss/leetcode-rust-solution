use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::majority_element_half(vec![1, 2, 5, 9, 5, 9, 5, 5, 5]),
        5
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::majority_element_half(vec![3, 3, 4]), 3);
}

#[test]
fn case_3() {
    assert_eq!(Solution::majority_element_half(vec![3, 2]), -1);
}

#[test]
fn case_4() {
    assert_eq!(Solution::majority_element_half(vec![1]), 1);
}

fn check_output_one_third(input: Vec<i32>, expect: Vec<i32>) {
    let mut output = Solution::majority_element_one_third(input);
    let mut expect = expect;
    output.sort();
    expect.sort();
    assert_eq!(output, expect);
}

#[test]
fn case_one_third_1() {
    check_output_one_third(vv![3, 2, 3], vv![3]);
}

#[test]
fn case_one_third_2() {
    check_output_one_third(vv![1], vv![1]);
}

#[test]
fn case_one_third_3() {
    check_output_one_third(vv![1, 1, 1, 3, 3, 2, 2, 2], vv![1, 2]);
}
