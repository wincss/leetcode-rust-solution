use crate::*;

use serde_json::json;

#[test]
fn case_1() {
    assert_eq!(
        Solution::merge_trees(
            TreeNode::from_json(json!([1, 3, 2, 5])),
            TreeNode::from_json(json!([2, 1, 3, null, 4, null, 7])),
        ),
        TreeNode::from_json(json!([3, 4, 5, 5, 4, null, 7]))
    );
}
