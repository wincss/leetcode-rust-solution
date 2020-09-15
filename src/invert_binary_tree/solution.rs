use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(root) => {
                let mut root = root.borrow_mut();
                let mut inv = TreeNode::new(root.val);
                inv.right = Solution::invert_tree(root.left.take());
                inv.left = Solution::invert_tree(root.right.take());
                Some(Rc::new(RefCell::new(inv)))
            }
        }
    }
}
