use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::postorder_traversal(TreeNode::from_json(json!([1, null, 2, 3]))),
        vec![3, 2, 1]
    )
}
