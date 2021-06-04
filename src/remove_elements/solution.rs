use crate::*;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut root = Box::new(ListNode::new(0));
        let mut current = &mut root;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if node.val != val {
                current.next = Some(node);
                current = current.next.as_mut().unwrap();
            }
        }
        root.next
    }
}
