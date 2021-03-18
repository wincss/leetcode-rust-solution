use crate::*;

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut root = ListNode::new(0);
        root.next = head;
        let mut root = Some(Box::new(root));
        let mut before_tail = &mut root;
        for _ in 1..left {
            before_tail = &mut before_tail.as_mut().unwrap().next;
        }
        let mut reverse_tail = before_tail.as_mut().unwrap().next.take();
        let mut last_reversed = None;
        for _ in left..=right {
            let node = reverse_tail.as_mut().unwrap();
            let next = node.next.take();
            node.next = last_reversed.take();
            last_reversed = reverse_tail.take();
            reverse_tail = next;
        }
        before_tail.as_mut().unwrap().next = last_reversed.take();

        while before_tail.as_ref().unwrap().next.is_some() {
            before_tail = &mut before_tail.as_mut().unwrap().next;
        }
        before_tail.as_mut().unwrap().next = reverse_tail;
        root.unwrap().next
    }
}
