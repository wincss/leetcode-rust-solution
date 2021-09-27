use crate::*;

#[test]
fn case_1() {
    let input = TreeNode::from_json(json!([5, 2, 13]));
    let expect = TreeNode::from_json(json!([18, 20, 13]));
    assert_eq!(Solution::convert_bst(input), expect);
}

#[test]
fn case_2() {
    let input = TreeNode::from_json(json!([
        4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8
    ]));
    let expect = TreeNode::from_json(json!([
        30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8
    ]));
    assert_eq!(Solution::bst_to_gst(input), expect);
}
