use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_tilt(TreeNode::from_json(json!([1, 2, 3]))),
        1
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_tilt(TreeNode::from_json(json!([4, 2, 9, 3, 5, null, 7]))),
        15
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::find_tilt(TreeNode::from_json(json!([21, 7, 14, 1, 1, 2, 2, 3, 3]))),
        9
    );
}
