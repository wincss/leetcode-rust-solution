use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::min_diff_in_bst(TreeNode::from_json(json!([4, 2, 6, 1, 3]))),
        1
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::min_diff_in_bst(TreeNode::from_json(json!([1, 0, 48, null, null, 12, 49]))),
        1
    );
}
