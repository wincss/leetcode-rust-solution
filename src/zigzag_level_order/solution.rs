use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn walk(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, result: &mut Vec<Vec<i32>>) {
            if root.is_none() {
                return;
            }
            let root = root.unwrap();
            let mut root = root.borrow_mut();
            if depth == result.len() {
                result.push(vec![]);
            }
            if depth & 1 == 0 {
                result[depth].push(root.val);
            } else {
                result[depth].insert(0, root.val);
            }
            walk(root.left.take(), depth + 1, result);
            walk(root.right.take(), depth + 1, result);
        }
        let mut result = vec![];
        walk(root, 0, &mut result);
        result
    }
}
