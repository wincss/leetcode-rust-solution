use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn inorder_traversal(
            root: Option<Rc<RefCell<TreeNode>>>,
            last_num: &mut Option<i32>,
            result: &mut i32,
        ) {
            if root.is_none() {
                return;
            }
            let root = root.as_ref().unwrap().borrow();
            inorder_traversal(root.left.as_ref().map(Rc::clone), last_num, result);
            if last_num.is_some() {
                *result = std::cmp::min(*result, root.val - last_num.unwrap());
            }
            last_num.replace(root.val);
            inorder_traversal(root.right.as_ref().map(Rc::clone), last_num, result);
        }
        let mut last_num = None;
        let mut result = std::i32::MAX;
        inorder_traversal(root, &mut last_num, &mut result);
        result
    }
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_minimum_difference(root)
    }
}
