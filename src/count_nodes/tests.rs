use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::count_nodes(TreeNode::from_json(json!([1, 2, 3, 4, 5, 6]))),
        6
    )
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::count_nodes(TreeNode::from_json(json!([1, 2, 3, 4, 5]))),
        5
    )
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::count_nodes(TreeNode::from_json(json!([1, 2, 3, 4]))),
        4
    )
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::count_nodes(TreeNode::from_json(json!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]))),
        10
    )
}

#[test]
fn case_5() {
    assert_eq!(Solution::count_nodes(None), 0)
}
