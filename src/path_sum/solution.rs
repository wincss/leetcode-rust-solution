use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        fn walk(
            root: Option<Rc<RefCell<TreeNode>>>,
            remain: i32,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if root.is_none() {
                return;
            }
            let mut node = root.as_ref().unwrap().borrow_mut();
            current.push(node.val);
            if node.left.is_none() && node.right.is_none() {
                if node.val == remain {
                    result.push(current.clone());
                }
            } else {
                walk(node.left.take(), remain - node.val, current, result);
                walk(node.right.take(), remain - node.val, current, result);
            }
            current.pop();
        }
        let mut result = vec![];
        let mut current = vec![];
        walk(root, sum, &mut current, &mut result);
        result
    }
}
