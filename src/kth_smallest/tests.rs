use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::kth_smallest(TreeNode::from_json(json!([3, 1, 4, null, 2])), 1),
        1
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::kth_smallest(
            TreeNode::from_json(json!([5, 3, 6, 2, 4, null, null, 1])),
            3
        ),
        3
    );
}
