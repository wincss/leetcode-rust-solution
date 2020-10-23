use crate::*;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut last = None;
        while head.is_some() {
            let next = head.as_mut().unwrap().next.take();
            head.as_mut().unwrap().next = last;
            last = head;
            head = next;
        }
        last
    }
}
