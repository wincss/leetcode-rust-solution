use crate::*;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn get_child(child: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            child
                .as_ref()
                .and_then(|v: &Rc<RefCell<TreeNode>>| Some(Rc::clone(&v)))
        }
        match root {
            None => None,
            Some(root) => {
                let root = root.borrow();
                let mut inv = TreeNode::new(root.val);
                inv.right = Solution::invert_tree(get_child(&root.left));
                inv.left = Solution::invert_tree(get_child(&root.right));
                Some(Rc::new(RefCell::new(inv)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from_array(&[
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]);
        let inverted_tree = TreeNode::from_array(&[
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1),
        ]);
        assert_eq!(Solution::invert_tree(tree), inverted_tree);
    }
}
