use crate::*;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut smaller = Some(Box::new(ListNode::new(0)));
        let mut larger = Some(Box::new(ListNode::new(0)));
        let mut smaller_tail = &mut smaller;
        let mut larger_tail = &mut larger;
        while let Some(mut item) = head {
            head = item.next.take();
            if item.val < x {
                smaller_tail.as_mut().unwrap().next.replace(item);
                smaller_tail = &mut smaller_tail.as_mut().unwrap().next;
            } else {
                larger_tail.as_mut().unwrap().next.replace(item);
                larger_tail = &mut larger_tail.as_mut().unwrap().next;
            }
        }
        smaller_tail.as_mut().unwrap().next = larger.take().unwrap().next;
        smaller.as_mut().unwrap().next.take()
    }
}
