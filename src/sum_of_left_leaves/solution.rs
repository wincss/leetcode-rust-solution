use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn traversal(root: Option<Rc<RefCell<TreeNode>>>, is_left: bool, result: &mut i32) {
            if root.is_none() {
                return;
            }
            let mut root = root.as_ref().unwrap().borrow_mut();
            if root.left.is_none() && root.right.is_none() && is_left {
                *result += root.val;
            }
            traversal(root.left.take(), true, result);
            traversal(root.right.take(), false, result);
        }
        let mut result = 0;
        traversal(root, false, &mut result);
        result
    }
}
