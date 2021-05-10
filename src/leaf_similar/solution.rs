use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

struct LeafIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl LeafIterator {
    pub fn new(root: &Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = vec![];
        if let Some(v) = root.as_ref() {
            stack.push(Rc::clone(v));
        }
        Self { stack }
    }
}

impl Iterator for LeafIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            let node = node.borrow();
            if node.right.is_some() {
                self.stack.push(Rc::clone(node.right.as_ref().unwrap()));
            }
            if node.left.is_some() {
                self.stack.push(Rc::clone(node.left.as_ref().unwrap()));
            }
            if node.left.is_none() && node.right.is_none() {
                return Some(node.val);
            }
        }
        None
    }
}

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        LeafIterator::new(&root1).eq(LeafIterator::new(&root2))
    }
}
