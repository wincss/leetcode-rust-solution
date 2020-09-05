use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        fn go(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, result: &mut Vec<Vec<i32>>) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            if depth >= result.len() {
                result.push(vec![]);
            }
            result[depth].push(node.val);
            go(node.left.as_ref().map(Rc::clone), depth + 1, result);
            go(node.right.as_ref().map(Rc::clone), depth + 1, result);
        }
        go(root, 0, &mut result);
        result.reverse();
        result
    }
}
