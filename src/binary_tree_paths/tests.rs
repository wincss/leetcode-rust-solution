use crate::*;

use std::collections::HashSet;
#[test]
fn case_1() {
    let tree = TreeNode::from_array(&[Some(1), Some(2), Some(3), None, Some(5)]);
    let output = (vec![String::from("1->2->5"), String::from("1->3")])
        .into_iter()
        .collect::<HashSet<String>>();
    assert_eq!(
        Solution::binary_tree_paths(tree)
            .into_iter()
            .collect::<HashSet<String>>(),
        output
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::binary_tree_paths(None), Vec::<String>::new());
}
