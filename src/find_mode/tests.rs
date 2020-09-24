use crate::*;

use serde_json::json;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_mode(TreeNode::from_json(json!([1, null, 2, 2]))),
        vec![2]
    );
}
