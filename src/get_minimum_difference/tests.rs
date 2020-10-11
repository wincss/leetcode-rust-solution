use crate::*;

use serde_json::json;

#[test]
fn case_1() {
    assert_eq!(
        Solution::get_minimum_difference(TreeNode::from_json(json!([1, null, 3, 2]))),
        1
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::get_minimum_difference(TreeNode::from_json(json!([4, 2, 6, 1, 3, null, null]))),
        1
    );
}
