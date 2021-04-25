use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, put: &mut Option<Rc<RefCell<TreeNode>>>) {
            if root.is_none() {
                return;
            }
            let root = root.as_ref().unwrap().borrow();
            inorder(root.left.as_ref().map(Rc::clone), put);
            let node = Rc::new(RefCell::new(TreeNode::new(root.val)));
            put.as_ref()
                .unwrap()
                .borrow_mut()
                .right
                .replace(Rc::clone(&node));
            put.replace(node);
            inorder(root.right.as_ref().map(Rc::clone), put)
        }
        let result = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let mut current = result.as_ref().map(Rc::clone);

        inorder(root, &mut current);
        let mut v = result.as_ref().unwrap().borrow_mut();
        v.right.take()
    }
}
