use crate::*;

use serde_json::json;

#[test]
fn case_1() {
    assert_eq!(
        Solution::zigzag_level_order(TreeNode::from_json(json!([3, 9, 20, null, null, 15, 7]))),
        vec![vec![3], vec![20, 9], vec![15, 7]]
    );
}
