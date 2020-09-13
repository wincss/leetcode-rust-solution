use crate::*;

use serde_json::json;

#[test]
fn case_1() {
    assert_eq!(
        Solution::inorder_traversal(TreeNode::from_json(json!([1, null, 2, 3]))),
        vec![1, 3, 2]
    )
}
