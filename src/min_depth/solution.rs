use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            if root.is_none() {
                return None;
            }
            let root = root.as_ref().unwrap().borrow();
            let left = helper(&root.left);
            let right = helper(&root.right);
            Some(
                1 + left
                    .xor(right)
                    .or_else(|| left.and_then(|v| Some(std::cmp::min(v, right.unwrap()))))
                    .unwrap_or(0),
            )
        }
        helper(&root).unwrap_or(0)
    }
}
