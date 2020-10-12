use crate::*;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = ListNode::new(0);
        root.next = head;
        let mut root = Box::new(root);
        let mut this = &mut root;
        while this.next.is_some() && this.next.as_ref().unwrap().next.is_some() {
            let mut a = this.next.take().unwrap();
            let mut b = a.next.take().unwrap();
            a.next = b.next.take();
            b.next.replace(a);
            this.next.replace(b);
            this = this.next.as_mut().unwrap().next.as_mut().unwrap();
        }
        root.next
    }
}
