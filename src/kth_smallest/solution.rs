use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

use common::InorderIterator;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut iter = InorderIterator::new(&root);
        iter.nth(k as usize - 1).unwrap()
    }
}
