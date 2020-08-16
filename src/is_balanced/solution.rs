use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn balanced_height(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            let ret = match root.as_ref() {
                None => Some(0),
                Some(tree) => {
                    let mut tree = tree.borrow_mut();
                    balanced_height(tree.left.take()).and_then(|left_height| {
                        balanced_height(tree.right.take()).and_then(|right_height| {
                            if (left_height - right_height).abs() <= 1 {
                                Some(1 + std::cmp::max(left_height, right_height))
                            } else {
                                None
                            }
                        })
                    })
                }
            };
            ret
        }
        balanced_height(root).is_some()
    }
}
