use crate::*;
use serde_json::json;

#[test]
fn case_1() {
    assert_eq!(
        Solution::increasing_bst(TreeNode::from_json(json!([
            5, 3, 6, 2, 4, null, 8, 1, null, null, null, 7, 9
        ]))),
        TreeNode::from_json(json!([
            1, null, 2, null, 3, null, 4, null, 5, null, 6, null, 7, null, 8, null, 9
        ]))
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::increasing_bst(TreeNode::from_json(json!([5, 1, 7]))),
        TreeNode::from_json(json!([1, null, 5, null, 7]))
    );
}
