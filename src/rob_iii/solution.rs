use crate::*;

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

impl Solution {
    pub fn rob_iii(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if root.is_none() {
                return (0, 0);
            }
            let root = root.as_ref().unwrap().borrow();
            let (left_rob, left_rest) = helper(&root.left);
            let (right_rob, right_rest) = helper(&root.right);

            (
                root.val + left_rest + right_rest,
                cmp::max(left_rob, left_rest) + cmp::max(right_rob, right_rest),
            )
        }
        let (x, y) = helper(&root);
        cmp::max(x, y)
    }
}
