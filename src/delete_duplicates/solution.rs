use crate::*;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut root;
        let mut last = None;
        let mut head = head;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if last.is_some() && last.unwrap() == node.val {
                continue;
            }

            last = Some(node.val);
            tail.as_mut().unwrap().next = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }
        root.unwrap().next
    }

    pub fn delete_duplicates_ii(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut root;
        let mut last: Option<Box<ListNode>> = None;
        let mut head = head;

        let mut need_remove = false;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if let Some(last_node) = last.as_ref() {
                let new_need_remove = last_node.val == node.val;
                if !new_need_remove && !need_remove {
                    tail.as_mut().unwrap().next = last.take();
                    tail = &mut tail.as_mut().unwrap().next;
                }
                need_remove = new_need_remove;
            }
            last.replace(node);
        }
        if !need_remove {
            tail.as_mut().unwrap().next = last;
        }
        root.unwrap().next
    }
}
