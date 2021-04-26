use crate::*;

use serde_json::json;

#[test]
fn case_1() {
    assert_eq!(
        Solution::range_sum_bst(
            TreeNode::from_json(json!([10, 5, 15, 3, 7, null, 18])),
            7,
            15
        ),
        32
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::range_sum_bst(
            TreeNode::from_json(json!([10, 5, 15, 3, 7, 13, 18, 1, null, 6])),
            6,
            10
        ),
        23
    );
}
