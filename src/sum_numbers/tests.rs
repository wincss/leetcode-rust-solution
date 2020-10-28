use crate::*;

use serde_json::json;

#[test]
fn case_1() {
    assert_eq!(
        Solution::sum_numbers(TreeNode::from_json(json!([1, 2, 3]))),
        25
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::sum_numbers(TreeNode::from_json(json!([4, 9, 0, 5, 1]))),
        1026
    );
}
