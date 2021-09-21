use crate::common::InorderIterator;
use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let iter = InorderIterator::new(&root);
        let mut last = None;
        for i in iter {
            if last.is_none() || last.unwrap() < i {
                last = Some(i);
            } else {
                return false;
            }
        }
        true
    }
}
