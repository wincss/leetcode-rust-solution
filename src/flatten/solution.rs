use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut current: Option<Rc<RefCell<TreeNode>>> = None;
        let mut left_stack = vec![];
        let mut right_stack = vec![];
        left_stack.push(Rc::clone(root.as_ref().unwrap()));
        while !left_stack.is_empty() || !right_stack.is_empty() {
            let vf = left_stack.pop().or_else(|| right_stack.pop()).unwrap();
            if current.is_some() {
                current
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .right
                    .replace(Rc::clone(&vf));
            }
            current.replace(Rc::clone(&vf));
            let mut v = vf.borrow_mut();
            if let Some(left) = v.left.take() {
                left_stack.push(Rc::clone(&left));
            }
            if let Some(right) = v.right.take() {
                right_stack.push(Rc::clone(&right));
            }
            // print!("{},", v.val);
        }
    }
}
