use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::average_of_levels(TreeNode::from_json(json!([3, 9, 20, 15, 7]))),
        vec![3.0, 14.5, 11.0]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::average_of_levels(TreeNode::from_json(json!([
            2147483647, 2147483647, 2147483647
        ]))),
        vec![2147483647.0, 2147483647.0]
    );
}
