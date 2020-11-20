use crate::*;

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = Some(Box::new(ListNode::new(std::i32::MIN)));
        let mut head = head;
        while let Some(mut v) = head.take() {
            head = v.next.take();
            let mut next = &mut result;
            while next.is_some() && next.as_ref().unwrap().val < v.val {
                next = &mut next.as_mut().unwrap().next;
            }
            v.next = next.take();
            next.replace(v);
        }
        result.take().unwrap().next
    }
}
