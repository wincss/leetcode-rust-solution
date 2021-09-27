use crate::*;

#[test]
fn case_1() {
    assert!(Solution::has_path_sum(
        TreeNode::from_json(json!([5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1])),
        22
    ));
}

#[test]
fn case_2() {
    assert!(!Solution::has_path_sum(
        TreeNode::from_json(json!([1, 2, 3])),
        5
    ));
}

#[test]
fn case_3() {
    assert!(!Solution::has_path_sum(
        TreeNode::from_json(json!([1, 2])),
        0
    ));
}
