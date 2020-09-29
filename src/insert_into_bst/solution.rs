use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn insert(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
            if root.is_none() {
                root.replace(Rc::new(RefCell::new(TreeNode::new(val))));
                return;
            }
            let mut node = root.as_ref().unwrap().borrow_mut();
            if val < node.val {
                insert(&mut node.left, val);
            } else {
                insert(&mut node.right, val);
            }
        }
        let mut root = root;
        insert(&mut root, val);
        root
    }
}
