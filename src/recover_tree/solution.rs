use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Rc<RefCell<TreeNode>>> {
            if root.is_none() {
                return vec![];
            }
            let root = root.as_ref().unwrap();
            let mut ret: Vec<Rc<RefCell<TreeNode>>> = vec![];
            ret.extend(inorder(&root.borrow().left).into_iter());
            ret.push(Rc::clone(root));
            ret.extend(inorder(&root.borrow().right).into_iter());
            ret
        }
        let mut first = None;
        let mut second = None;
        let mut last = None;
        let mut last_val = std::i32::MIN;
        for i in inorder(root).into_iter() {
            let node = i.borrow();
            if last_val > node.val {
                if first.is_none() {
                    first = last.take();
                    second.replace(Rc::clone(&i));
                } else {
                    second.replace(Rc::clone(&i));
                }
            }
            last_val = node.val;
            last.replace(Rc::clone(&i));
        }
        let first = first.unwrap();
        let second = second.unwrap();
        let mut x = first.borrow_mut();
        let mut y = second.borrow_mut();
        let tmp = x.val;
        x.val = y.val;
        y.val = tmp;
    }
}
