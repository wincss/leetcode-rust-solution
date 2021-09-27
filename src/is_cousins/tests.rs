use crate::*;

#[test]
fn case_1() {
    assert!(!Solution::is_cousins(
        TreeNode::from_json(json!([1, 2, 3, 4])),
        4,
        3
    ));
}

#[test]
fn case_2() {
    assert!(Solution::is_cousins(
        TreeNode::from_json(json!([1, 2, 3, null, 4, null, 5])),
        5,
        4
    ));
}

#[test]
fn case_3() {
    assert!(!Solution::is_cousins(
        TreeNode::from_json(json!([1, 2, 3, null, 4])),
        2,
        3
    ));
}
