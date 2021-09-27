use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::level_order_bottom(TreeNode::from_json(json!([3, 9, 20, null, null, 15, 7]))),
        vec![vec![15, 7], vec![9, 20], vec![3]]
    );
}
