use crate::*;

use std::collections::VecDeque;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        fn merge(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut head = Box::new(ListNode::new(0));

            let mut next = Some(&mut head);
            let mut l1 = l1;
            let mut l2 = l2;
            while l1.is_some() || l2.is_some() {
                if l1.is_none() {
                    next.unwrap().next = l2;
                    break;
                }
                if l2.is_none() {
                    next.unwrap().next = l1;
                    break;
                }
                let mut v1 = l1.take().unwrap();
                let mut v2 = l2.take().unwrap();
                if v1.val < v2.val {
                    l1 = v1.next.take();
                    l2.replace(v2);

                    let new = next.take().unwrap();
                    new.next = Some(v1);
                    next = new.next.as_mut();
                } else {
                    l2 = v2.next.take();
                    l1.replace(v1);

                    let new = next.take().unwrap();
                    new.next = Some(v2);
                    next = new.next.as_mut();
                }
            }
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
