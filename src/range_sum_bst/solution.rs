use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut sum = 0;
        let root = root.as_ref().unwrap().borrow();
        if root.val >= low && root.val <= high {
            sum += root.val;
        }
        if root.val > low {
            sum += Self::range_sum_bst(root.left.as_ref().map(Rc::clone), low, high);
        }
        if root.val < high {
            sum += Self::range_sum_bst(root.right.as_ref().map(Rc::clone), low, high);
        }
        sum
    }
}
