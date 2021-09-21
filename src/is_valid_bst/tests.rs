use crate::*;

use serde_json::json;

#[test]
fn case_1() {
    assert!(Solution::is_valid_bst(TreeNode::from_json(json!([
        2, 1, 3
    ]))));
}

#[test]
fn case_2() {
    assert!(!Solution::is_valid_bst(TreeNode::from_json(json!([
        5, 1, 4, null, null, 3, 6
    ]))));
}

#[test]
fn case_3() {
    assert!(!Solution::is_valid_bst(TreeNode::from_json(json!([
        5, 4, 6, null, null, 3, 7
    ]))));
}
