use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::path_sum_113(
        TreeNode::from_json(json!([5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1])),
        22,
    );
    output.sort();
    let mut expect = vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]];
    expect.sort();
    assert_eq!(output, expect);
}

#[test]
fn case_2() {
    let mut output = Solution::path_sum_113(TreeNode::from_json(json!([-2, null, -3])), -5);
    output.sort();
    let mut expect = vec![vec![-2, -3]];
    expect.sort();
    assert_eq!(output, expect);
}

#[test]
fn case_437_1() {
    assert_eq!(
        Solution::path_sum_437(
            TreeNode::from_json(json!([10, 5, -3, 3, 2, null, 11, 3, -2, null, 1])),
            8
        ),
        3
    );
}

#[test]
fn case_437_2() {
    assert_eq!(
        Solution::path_sum_437(
            TreeNode::from_json(json!([5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1])),
            22
        ),
        3
    );
}

#[test]
fn case_437_3() {
    assert_eq!(
        Solution::path_sum_437(
            TreeNode::from_json(json!([5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1])),
            22
        ),
        3
    );
}

#[test]
fn case_437_4() {
    assert_eq!(
        Solution::path_sum_437(
            TreeNode::from_json(json!([10, 5, -3, 3, 24, null, 11, 3, -2, null, 1])),
            21
        ),
        1
    );
}
