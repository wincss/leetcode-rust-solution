pub struct Solution;
pub mod flatten;
pub mod invert_binary_tree;
pub mod minimal_steps;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Display;
use std::iter::FromIterator;
use std::rc::Rc;

pub fn to_string_vec<T: Display>(list: &[T]) -> Vec<String> {
    let mut ret = vec![];
    for i in list.iter() {
        ret.push(i.to_string());
    }
    ret
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn to_tree(list: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut list: VecDeque<Option<i32>> = VecDeque::from_iter(list.to_vec());
    let mut queue = VecDeque::new();
    let mut root = None;
    while !list.is_empty() {
        if root.is_none() {
            let node = Rc::new(RefCell::new(TreeNode::new(
                list.pop_front().unwrap().expect("root must not null"),
            )));
            root.replace(Rc::clone(&node));
            queue.push_back(Rc::clone(&node));
            continue;
        }
        let current = queue.pop_front().expect("too many items");
        if let Some(v) = list.pop_front().unwrap() {
            let node = Rc::new(RefCell::new(TreeNode::new(v)));
            current.borrow_mut().left.replace(Rc::clone(&node));
            queue.push_back(Rc::clone(&node));
        }
        if let Some(v) = list.pop_front().flatten() {
            let node = Rc::new(RefCell::new(TreeNode::new(v)));
            current.borrow_mut().right.replace(Rc::clone(&node));
            queue.push_back(Rc::clone(&node));
        }
    }
    root
}
