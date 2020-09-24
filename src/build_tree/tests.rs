use crate::*;

use serde_json::json;

#[test]
fn case_1() {
    assert_eq!(
        Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
        TreeNode::from_json(json!([3, 9, 20, null, null, 15, 7]))
    )
}
