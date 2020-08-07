use crate::*;

#[test]
fn case_1() {
    let mut tree = TreeNode::from_array(&[Some(1), Some(3), None, None, Some(2)]);
    Solution::recover_tree(&mut tree);
    assert_eq!(
        tree,
        TreeNode::from_array(&[Some(3), Some(1), None, None, Some(2)])
    );
}

#[test]
fn case_2() {
    let mut tree = TreeNode::from_array(&[Some(3), Some(1), Some(4), None, None, Some(2)]);
    Solution::recover_tree(&mut tree);
    assert_eq!(
        tree,
        TreeNode::from_array(&[Some(2), Some(1), Some(4), None, None, Some(3)])
    );
}
