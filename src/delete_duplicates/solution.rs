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
        let mut root = ListNode::new(0);
        root.next = head;
        let mut root = Box::new(root);
        let mut confirmed_tail = &mut root;
        while let Some(mut n1) = confirmed_tail.next.take() {
            let mut need_remove = false;
            while let Some(n2) = n1.next.take() {
                if n1.val == n2.val {
                    need_remove = true;
                    n1.next = n2.next;
                } else {
                    n1.next.replace(n2);
                    break;
                }
            }
            if need_remove {
                confirmed_tail.next = n1.next;
            } else {
                confirmed_tail.next.replace(n1);
                confirmed_tail = confirmed_tail.next.as_mut().unwrap();
            }
        }
        root.next
    }
}
