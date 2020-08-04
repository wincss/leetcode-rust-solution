use crate::*;

#[test]
fn case_1() {
    let mut tree =
        TreeNode::from_array(&[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)]);
    let flattened_tree = TreeNode::from_array(&[
        Some(1),
        None,
        Some(2),
        None,
        Some(3),
        None,
        Some(4),
        None,
        Some(5),
        None,
        Some(6),
    ]);
    Solution::flatten(&mut tree);
    assert_eq!(tree, flattened_tree);
}

#[test]
fn case_2() {
    let mut tree = TreeNode::from_array(&[]);
    Solution::flatten(&mut tree);
    assert_eq!(tree, None);
}

#[test]
fn case_3() {
    let mut tree = TreeNode::from_array(&[Some(0)]);
    Solution::flatten(&mut tree);
    assert_eq!(tree, TreeNode::from_array(&[Some(0)]));
}
