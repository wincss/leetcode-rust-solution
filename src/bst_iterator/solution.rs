use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
    next_item: Option<i32>,
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    #[allow(dead_code)]
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iter = Self {
            stack: vec![],
            next_item: None,
        };
        if root.is_some() {
            iter.stack.push(root.unwrap());
        }
        iter.prefetch();
        iter
    }

    #[allow(dead_code)]
    pub fn next(&mut self) -> i32 {
        let tmp = self.next_item.take().unwrap();
        self.prefetch();
        tmp
    }

    #[allow(dead_code)]
    pub fn has_next(&self) -> bool {
        self.next_item.is_some()
    }

    fn prefetch(&mut self) {
        while let Some(node) = self.stack.pop() {
            let left_option = node.borrow_mut().left.take();
            if let Some(left_child) = left_option {
                self.stack.push(node);
                self.stack.push(left_child);
            } else {
                self.next_item = Some(node.borrow().val);
                if let Some(right_child) = node.borrow_mut().right.take() {
                    self.stack.push(right_child);
                }
                break;
            }
        }
    }
}
