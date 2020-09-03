use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.is_none() || q.is_none() {
            return false;
        }
        let p = p.as_ref().unwrap().borrow();
        let q = q.as_ref().unwrap().borrow();
        return p.val == q.val
            && Solution::is_same_tree(
                p.left.as_ref().map(Rc::clone),
                q.left.as_ref().map(Rc::clone),
            )
            && Solution::is_same_tree(
                p.right.as_ref().map(Rc::clone),
                q.right.as_ref().map(Rc::clone),
            );

        // let mut p = p.as_ref().unwrap().borrow_mut();
        // let mut q = q.as_ref().unwrap().borrow_mut();
        // return p.val == q.val
        //     && Solution::is_same_tree(p.left.take(), q.left.take())
        //     && Solution::is_same_tree(p.right.take(), q.right.take());
    }
}
