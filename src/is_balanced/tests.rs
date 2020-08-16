use crate::*;

#[test]
fn case_1() {
    let tree = TreeNode::from_array(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert!(Solution::is_balanced(tree))
}

#[test]
fn case_2() {
    let tree = TreeNode::from_array(&[
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(3),
        None,
        None,
        Some(4),
        Some(4),
    ]);
    assert!(!Solution::is_balanced(tree))
}
