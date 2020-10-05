use crate::*;

fn check_result(input: Vec<i32>, target: i32, expect: Vec<Vec<i32>>) {
    let mut output = Solution::four_sum(input, target);
    for item in output.iter_mut() {
        item.sort();
    }
    output.sort();
    let mut expect = expect;
    for item in expect.iter_mut() {
        item.sort();
    }
    expect.sort();
    assert_eq!(output, expect);
}

#[test]
fn case_1() {
    check_result(
        vec![1, 0, -1, 0, -2, 2],
        0,
        vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
    );
}
