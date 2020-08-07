use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[test]
#[ignore]
fn create_treenode() {
    let third = TreeNode::new(3);
    let second = TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(third))),
        right: None,
    };
    let first = TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(second))),
    };

    assert_eq!(
        TreeNode::from_array(&[Some(1), None, Some(2), Some(3)]),
        Some(Rc::new(RefCell::new(first)))
    );
}

#[test]
#[ignore]
fn treenode_to_vec() {
    let root = TreeNode::from_array(&[Some(1), None, Some(2), Some(3)]);
    assert_eq!(
        root.as_ref().unwrap().borrow().to_vec(),
        vec![Some(1), None, Some(2), Some(3)]
    );
}

#[test]
#[ignore]
fn display_treenode() {
    let root = TreeNode::from_array(&[Some(1), None, Some(2), Some(3)]);
    assert_eq!(
        format!("{}", root.as_ref().unwrap().borrow()),
        "[1, null, 2, 3]".to_string()
    );
}

#[test]
#[ignore]
fn preorder_traversal() {
    let root = TreeNode::from_array(&[Some(1), None, Some(2), Some(3)]);
    assert_eq!(root.unwrap().borrow().preorder(), vec![1, 2, 3]);
}

#[test]
#[ignore]
fn inorder_traversal() {
    let root = TreeNode::from_array(&[Some(1), None, Some(2), Some(3)]);
    assert_eq!(root.unwrap().borrow().inorder(), vec![1, 3, 2]);
}

#[test]
#[ignore]
fn postorder_traversal() {
    let root = TreeNode::from_array(&[Some(1), None, Some(2), Some(3)]);
    assert_eq!(root.unwrap().borrow().postorder(), vec![3, 2, 1]);
}
