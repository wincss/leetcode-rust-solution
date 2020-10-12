use crate::*;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = ListNode::new(0);
        root.next = head;
        let mut root = Box::new(root);
        let mut this = &mut root;
        while let Some(mut a) = this.next.take() {
            if let Some(mut b) = a.next.take() {
                a.next = b.next.take();
                b.next.replace(a);
                this.next.replace(b);
                this = this.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                this.next.replace(a);
                break;
            }
        }
        root.next
    }
}
