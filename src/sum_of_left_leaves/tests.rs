use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::sum_of_left_leaves(TreeNode::from_json(json!([3, 9, 20, null, null, 15, 7]))),
        24
    );
}
