use crate::*;

#[test]
fn case_1() {
    assert!(Solution::leaf_similar(
        TreeNode::from_json(json!([3, 5, 1, 6, 2, 9, 8, null, null, 7, 4])),
        TreeNode::from_json(json!([
            3, 5, 1, 6, 7, 4, 2, null, null, null, null, null, null, 9, 8
        ]))
    ));
}

#[test]
fn case_2() {
    assert!(Solution::leaf_similar(
        TreeNode::from_json(json!([1])),
        TreeNode::from_json(json!([1]))
    ));
}

#[test]
fn case_3() {
    assert!(!Solution::leaf_similar(
        TreeNode::from_json(json!([1])),
        TreeNode::from_json(json!([2]))
    ));
}

#[test]
fn case_4() {
    assert!(Solution::leaf_similar(
        TreeNode::from_json(json!([1, 2])),
        TreeNode::from_json(json!([2, 2]))
    ));
}

#[test]
fn case_5() {
    assert!(!Solution::leaf_similar(
        TreeNode::from_json(json!([1, 2, 3])),
        TreeNode::from_json(json!([1, 3, 2]))
    ));
}
