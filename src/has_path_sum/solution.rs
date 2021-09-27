use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn walk(root: Option<Rc<RefCell<TreeNode>>>, remain: i32) -> bool {
            if root.is_none() {
                return false;
            }
            let node = root.as_ref().unwrap().borrow();
            if node.left.is_none() && node.right.is_none() {
                node.val == remain
            } else {
                walk(node.left.as_ref().map(Rc::clone), remain - node.val)
                    || walk(node.right.as_ref().map(Rc::clone), remain - node.val)
            }
        }
        walk(root, target_sum)
    }
}
