use crate::*;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
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
}
