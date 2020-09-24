use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            let n = postorder.len();
            if n == 0 {
                return None;
            }
            let root_val = postorder[n - 1];
            let mut root = TreeNode::new(root_val);
            let mut position = 0;
            for i in 0..n {
                if inorder[i] == root_val {
                    position = i;
                }
            }
            let left_inorder = &inorder[..position];
            let right_inorder = &inorder[position + 1..];
            let left_postorder = &postorder[..left_inorder.len()];
            let right_postorder = &postorder[left_inorder.len()..n - 1];
            root.left = build(left_inorder, left_postorder);
            root.right = build(right_inorder, right_postorder);
            Some(Rc::new(RefCell::new(root)))
        }
        build(&inorder[..], &postorder[..])
    }
}
