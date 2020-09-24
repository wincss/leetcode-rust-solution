use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn inorder_traversal(
            root: Option<Rc<RefCell<TreeNode>>>,
            last_num: &mut i32,
            last_count: &mut i32,
            result: &mut Vec<i32>,
            result_count: &mut i32,
        ) {
            if root.is_none() {
                return;
            }
            let root = root.as_ref().unwrap().borrow();
            inorder_traversal(
                root.left.as_ref().map(Rc::clone),
                last_num,
                last_count,
                result,
                result_count,
            );
            if root.val == *last_num {
                *last_count += 1;
            } else {
                *last_num = root.val;
                *last_count = 1;
            }
            if last_count > result_count {
                result.clear();
                result.push(*last_num);
                *result_count = *last_count;
            } else if last_count == result_count {
                result.push(*last_num);
            }
            inorder_traversal(
                root.right.as_ref().map(Rc::clone),
                last_num,
                last_count,
                result,
                result_count,
            );
        }
        let mut last_num = 0;
        let mut last_count = 0;
        let mut result = vec![];
        let mut result_count = 0;
        inorder_traversal(
            root,
            &mut last_num,
            &mut last_count,
            &mut result,
            &mut result_count,
        );
        result
    }
}
