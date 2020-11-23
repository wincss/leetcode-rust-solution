use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut height = 0;
        let mut p = root.as_ref().map(Rc::clone);
        while let Some(v) = p {
            height += 1;
            p = v.borrow().left.as_ref().map(Rc::clone);
        }
        if height == 0 {
            return 0;
        }
        let mut l = 0;
        let mut r = 2_i32.pow(height - 1) - 1;
        while l < r {
            let m = l + (r - l + 1) / 2;

            let mut p = root.as_ref().map(Rc::clone);
            let mut found = true;
            for i in (0..height - 1).rev() {
                if m & (2_i32.pow(i)) == 0 {
                    p = p.unwrap().borrow().left.as_ref().map(Rc::clone);
                } else {
                    p = p.unwrap().borrow().right.as_ref().map(Rc::clone);
                }
                if p.is_none() {
                    found = false;
                    break;
                }
            }
            // println!("{},{},{},{}", l, m, r, found);

            if found {
                l = m;
            } else {
                r = m - 1;
            }
        }
        2_i32.pow(height - 1) + l
    }
}
