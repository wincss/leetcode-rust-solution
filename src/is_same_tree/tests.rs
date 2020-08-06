use crate::*;

#[test]
fn case_1() {
    let tree = TreeNode::from_array(&[
        Some(4),
        Some(2),
        Some(7),
        Some(1),
        Some(3),
        Some(6),
        Some(9),
    ]);
    let another_tree = TreeNode::from_array(&[
        Some(4),
        Some(2),
        Some(7),
        Some(1),
        Some(3),
        Some(6),
        Some(9),
    ]);
    assert!(Solution::is_same_tree(tree, another_tree));
}

#[test]
fn case_2() {
    assert!(!Solution::is_same_tree(
        TreeNode::from_array(&[Some(1), Some(2)]),
        TreeNode::from_array(&[Some(1), None, Some(2)])
    ));
}
