use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // See also: get_minimum_difference
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_diff = std::i32::MAX;
        let mut last_item = None;
        let mut stack = vec![root.unwrap()];
        while let Some(node) = stack.pop() {
            let left_option = node.borrow_mut().left.take();
            if let Some(left_child) = left_option {
                stack.push(node);
                stack.push(left_child);
            } else {
                let val = node.borrow().val;
                // println!("{}", val);
                if let Some(last_val) = last_item.replace(val) {
                    min_diff = min_diff.min(val - last_val);
                }
                if let Some(right_child) = node.borrow_mut().right.take() {
                    stack.push(right_child);
                }
            }
        }
        min_diff
    }
}
