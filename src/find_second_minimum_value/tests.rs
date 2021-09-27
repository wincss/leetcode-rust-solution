use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_second_minimum_value(TreeNode::from_json(json!([
            2, 2, 5, null, null, 5, 7
        ]))),
        5
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_second_minimum_value(TreeNode::from_json(json!([2, 2, 2]))),
        -1
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::find_second_minimum_value(TreeNode::from_json(json!([5, 8, 5]))),
        8
    );
}
