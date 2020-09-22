use crate::*;

use serde_json::json;

#[test]
fn case_1() {
    assert_eq!(
        Solution::min_camera_cover(TreeNode::from_json(json!([0, 0, null, 0, 0]))),
        1
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::min_camera_cover(TreeNode::from_json(json!([
            0, 0, null, 0, null, 0, null, null, 0
        ]))),
        2
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::min_camera_cover(TreeNode::from_json(json!([0]))),
        1
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::min_camera_cover(TreeNode::from_json(json!([0, 0, 0, null, null, null, 0]))),
        2
    );
}
