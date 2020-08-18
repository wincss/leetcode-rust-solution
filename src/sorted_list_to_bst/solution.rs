use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// https://leetcode-cn.com/problems/convert-sorted-list-to-binary-search-tree/
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(head: &Option<Box<ListNode>>, length: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if length == 0 {
                return None;
            }
            let mut curr = head;
            for _ in 0..length / 2 {
                curr = &curr.as_ref().unwrap().next;
            }
            let mut tree = TreeNode::new(curr.as_ref().unwrap().val);
            curr = &curr.as_ref().unwrap().next;
            tree.left = helper(head, length / 2).take();
            tree.right = helper(curr, length - length / 2 - 1).take();
            Some(Rc::new(RefCell::new(tree)))
        }
        let mut n = 0;
        let mut curr = &head;
        while let Some(v) = curr.as_ref() {
            n += 1;
            curr = &v.next;
        }
        helper(&head, n)
    }
}
