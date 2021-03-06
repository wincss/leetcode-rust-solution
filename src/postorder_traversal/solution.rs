use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        Self::postorder_traversal_recursion(root, &mut result);
        result
    }
    pub fn postorder_traversal_recursion(
        root: Option<Rc<RefCell<TreeNode>>>,
        result: &mut Vec<i32>,
    ) {
        if root.is_none() {
            return;
        }
        let root = root.as_ref().unwrap().borrow();
        Self::postorder_traversal_recursion(root.left.as_ref().map(Rc::clone), result);
        Self::postorder_traversal_recursion(root.right.as_ref().map(Rc::clone), result);
        result.push(root.val);
    }
}
