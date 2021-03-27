use crate::*;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut n = 0;
        let mut current = &head;
        while let Some(v) = current.as_ref() {
            n += 1;
            current = &v.next;
        }
        if n == 0 {
            return head;
        }
        let k = k % n;
        let mut current = &mut head;
        for _ in 1..n - k {
            current = &mut current.as_mut().unwrap().next;
        }
        let mut start = current.as_mut().unwrap().next.take();
        if start.is_none() {
            return head;
        }
        let mut current = &mut start;
        while current.as_ref().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }
        current.as_mut().unwrap().next = head;
        start
    }
}
