use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::inorder_traversal_recursion(root)
    }
    pub fn inorder_traversal_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = vec![];
        let root = root.as_ref().unwrap().borrow();
        ans.extend(Self::inorder_traversal_recursion(
            root.left.as_ref().map(Rc::clone),
        ));
        ans.push(root.val);
        ans.extend(Self::inorder_traversal_recursion(
            root.right.as_ref().map(Rc::clone),
        ));
        ans
    }
}
