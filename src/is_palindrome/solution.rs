use crate::*;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }
        let mut n = 0;
        {
            let mut root = &head;
            while root.is_some() {
                n += 1;
                root = &root.as_ref().unwrap().next;
            }
        }
        let mut head = head;
        let mut half = &mut head;
        for _ in 0..(n - 1) / 2 {
            half = &mut half.as_mut().unwrap().next;
        }
        let mut half = half.as_mut().unwrap().next.take();
        let mut last = None;
        while half.is_some() {
            let next = half.as_mut().unwrap().next.take();
            half.as_mut().unwrap().next = last;
            last = half;
            half = next;
        }
        while head.is_some() && last.is_some() {
            if head.as_ref().unwrap().val != last.as_ref().unwrap().val {
                return false;
            }
            head = head.as_mut().unwrap().next.take();
            last = last.as_mut().unwrap().next.take();
        }
        true
    }
}
