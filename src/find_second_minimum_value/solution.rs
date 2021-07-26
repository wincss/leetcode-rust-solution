use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min1 = None;
        let mut min2 = None;
        fn walk(
            root: Option<Rc<RefCell<TreeNode>>>,
            min1: &mut Option<i32>,
            min2: &mut Option<i32>,
        ) {
            if let Some(node) = root.as_ref() {
                let node = node.borrow();
                let val = node.val;
                if min1.is_none() {
                    min1.replace(val);
                } else if val < min1.unwrap() {
                    min2.replace(min1.take().unwrap());
                    min1.replace(val);
                } else if val > min1.unwrap() && (min2.is_none() || val < min2.unwrap()) {
                    min2.replace(val);
                }
                walk(node.left.as_ref().map(Rc::clone), min1, min2);
                walk(node.right.as_ref().map(Rc::clone), min1, min2);
            }
        }
        walk(root, &mut min1, &mut min2);
        min2.unwrap_or(-1)
    }
}
