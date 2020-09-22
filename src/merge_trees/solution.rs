use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if t1.is_none() && t2.is_none() {
            None
        } else if t1.is_none() {
            t2
        } else if t2.is_none() {
            t1
        } else {
            let mut n1 = t1.as_ref().unwrap().borrow_mut();
            let mut n2 = t2.as_ref().unwrap().borrow_mut();
            let mut n = TreeNode::new(n1.val + n2.val);
            n.left = Self::merge_trees(n1.left.take(), n2.left.take());
            n.right = Self::merge_trees(n1.right.take(), n2.right.take());
            Some(Rc::new(RefCell::new(n)))
        }
    }
}
