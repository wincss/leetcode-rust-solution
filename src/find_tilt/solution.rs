use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn walk(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
            if let Some(node) = root.as_ref() {
                let left = walk(node.borrow().left.as_ref().map(Rc::clone), result);
                let right = walk(node.borrow().right.as_ref().map(Rc::clone), result);
                *result += (left - right).abs();
                node.borrow().val + left + right
            } else {
                0
            }
        }
        let mut result = 0;
        walk(root, &mut result);
        result
    }
}
