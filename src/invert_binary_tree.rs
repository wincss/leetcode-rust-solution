use crate::*;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn get_child(child: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            child
                .as_ref()
                .and_then(|v: &Rc<RefCell<TreeNode>>| Some(Rc::clone(&v)))
        }
        match root {
            None => None,
            Some(root) => {
                let root = root.borrow();
                let mut inv = TreeNode::new(root.val);
                inv.right = Solution::invert_tree(get_child(&root.left));
                inv.left = Solution::invert_tree(get_child(&root.right));
                Some(Rc::new(RefCell::new(inv)))
            }
        }
    }
}
