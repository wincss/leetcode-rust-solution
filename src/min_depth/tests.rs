use crate::*;

#[test]
fn case_1() {
    let tree = TreeNode::from_array(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(Solution::min_depth(tree), 2);
}

#[test]
fn case_2() {
    let tree = TreeNode::from_array(&[]);
    assert_eq!(Solution::min_depth(tree), 0);
}

#[test]
fn case_3() {
    let tree = TreeNode::from_array(&[Some(1)]);
    assert_eq!(Solution::min_depth(tree), 1);
}

#[test]
fn case_4() {
    let tree = TreeNode::from_array(&[Some(1), Some(1)]);
    assert_eq!(Solution::min_depth(tree), 2);
}

#[test]
fn case_5() {
    let tree = TreeNode::from_array(&[Some(1), None, Some(1)]);
    assert_eq!(Solution::min_depth(tree), 2);
}
