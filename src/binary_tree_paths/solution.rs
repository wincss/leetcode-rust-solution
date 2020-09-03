use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result = vec![];
        let mut current = vec![];
        fn walk(
            root: Option<Rc<RefCell<TreeNode>>>,
            current: &mut Vec<String>,
            result: &mut Vec<String>,
        ) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            current.push(node.val.to_string());
            if node.left.is_none() && node.right.is_none() {
                result.push(current.join("->"));
            }
            walk(
                node.left.as_ref().and_then(|v| Some(Rc::clone(&v))),
                current,
                result,
            );
            walk(
                node.right.as_ref().and_then(|v| Some(Rc::clone(&v))),
                current,
                result,
            );
            current.pop();
        }
        walk(root, &mut current, &mut result);
        result
    }
}
