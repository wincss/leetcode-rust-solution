use crate::common::InorderIterator;
use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let iter = InorderIterator::new(&root);
        iter.scan(None, |last_opt, this| {
            last_opt
                .replace(this)
                .map_or(Some(true), |last| Some(last < this))
        })
        .all(|x| x)
    }
}
