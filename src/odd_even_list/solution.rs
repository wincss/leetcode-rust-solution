use crate::*;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut even = None;
        let mut even_ref = &mut even;
        let mut odd = None;
        let mut odd_ref = &mut odd;

        let mut index = 1;
        let mut head = head;
        while let Some(mut v) = head.take() {
            head = v.next.take();
            if index == 1 {
                odd_ref.replace(v);
                odd_ref = &mut odd_ref.as_mut().unwrap().next;
            } else {
                even_ref.replace(v);
                even_ref = &mut even_ref.as_mut().unwrap().next;
            }
            index ^= 1;
        }
        if even.is_some() {
            odd_ref.replace(even.take().unwrap());
        }
        odd
    }
}
