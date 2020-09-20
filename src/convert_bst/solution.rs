use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn convert(root: &Option<Rc<RefCell<TreeNode>>>, add: i32) -> i32 {
            if root.is_none() {
                return add;
            }
            let mut node = root.as_ref().unwrap().borrow_mut();
            node.val += convert(&node.right.as_ref().map(Rc::clone), add);
            convert(&node.left.as_ref().map(Rc::clone), node.val)
        }
        convert(&root, 0);
        root
    }

    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::convert_bst(root)
    }
}
