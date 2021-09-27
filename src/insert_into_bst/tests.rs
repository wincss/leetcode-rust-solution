use crate::*;

#[test]
fn case_1() {
    let input = TreeNode::from_json(json!([4, 2, 7, 1, 3]));
    let output = Solution::insert_into_bst(input, 5);
    let sequence = output.as_ref().unwrap().borrow().inorder();
    assert_eq!(sequence, vec![1, 2, 3, 4, 5, 7]);
}
