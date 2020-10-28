use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn walk(root: Option<Rc<RefCell<TreeNode>>>, parent: i32, result: &mut i32) {
            if root.is_none() {
                return;
            }
            let root = root.as_ref().unwrap().borrow();
            let parent = parent * 10 + root.val;
            if root.left.is_none() && root.right.is_none() {
                *result += parent;
            } else {
                walk(root.left.as_ref().map(Rc::clone), parent, result);
                walk(root.right.as_ref().map(Rc::clone), parent, result);
            }
        }
        let mut result = 0;
        walk(root, 0, &mut result);
        result
    }
}
