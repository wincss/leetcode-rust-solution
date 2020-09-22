use crate::*;

use std::collections::VecDeque;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        fn merge(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut head = Box::new(ListNode::new(0));

            let mut next = &mut head;
            let mut l1 = l1;
            let mut l2 = l2;
            while l1.is_some() && l2.is_some() {
                let mut v1 = l1.take().unwrap();
                let mut v2 = l2.take().unwrap();
                if v1.val < v2.val {
                    l1 = v1.next.take();
                    l2.replace(v2);

                    next.next = Some(v1);
                } else {
                    l2 = v2.next.take();
                    l1.replace(v1);

                    next.next = Some(v2);
                }
                next = next.next.as_mut().unwrap();
            }
            next.next = l1.xor(l2);
            head.next
        }
        let mut lists: VecDeque<Option<Box<ListNode>>> = lists.into_iter().collect();
        while let Some(l1) = lists.pop_front() {
            if let Some(l2) = lists.pop_front() {
                lists.push_back(merge(l1, l2));
            } else {
                return l1;
            }
        }
        None
    }
}
