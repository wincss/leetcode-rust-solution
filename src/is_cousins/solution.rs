use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        fn inorder(
            root: Option<Rc<RefCell<TreeNode>>>,
            x: i32,
            y: i32,
            parent: i32,
            height: i32,

            last_parent: &mut i32,
            last_height: &mut i32,
        ) -> bool {
            if root.is_none() {
                return false;
            }
            let root = Rc::clone(root.as_ref().unwrap());
            let root = root.borrow();
            let is_target = root.val == x || root.val == y;
            if is_target {
                // println!("this_parent = {} last_parent = {}", parent, last_parent);
                // println!("this_height = {} last_height = {}", height, last_height);
                if *last_parent > 0 {
                    return *last_parent != parent && *last_height == height;
                }
                *last_parent = parent;
                *last_height = height;
            }
            inorder(
                root.left.as_ref().map(Rc::clone),
                x,
                y,
                root.val,
                height + 1,
                last_parent,
                last_height,
            ) || inorder(
                root.right.as_ref().map(Rc::clone),
                x,
                y,
                root.val,
                height + 1,
                last_parent,
                last_height,
            )
        }
        let mut last_parent = 0;
        let mut last_height = 0;
        inorder(root, x, y, 0, 0, &mut last_parent, &mut last_height)
    }
}
