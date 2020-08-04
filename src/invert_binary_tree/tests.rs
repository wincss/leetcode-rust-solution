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
    let inverted_tree = TreeNode::from_array(&[
        Some(4),
        Some(7),
        Some(2),
        Some(9),
        Some(6),
        Some(3),
        Some(1),
    ]);
    assert_eq!(Solution::invert_tree(tree), inverted_tree);
}
