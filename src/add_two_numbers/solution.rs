use crate::*;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut ans = Box::new(ListNode::new(0));
        let mut head = &mut ans;
        let mut l1 = &l1;
        let mut l2 = &l2;
        loop {
            if let Some(val) = l1 {
                carry += val.val;
                l1 = &val.next;
            }
            if let Some(val) = l2 {
                carry += val.val;
                l2 = &val.next;
            }
            head.next = Some(Box::new(ListNode::new(carry % 10)));
            carry = carry / 10;
            head = head.next.as_mut().unwrap();
            // println!("{},{},{}",l1.is_none(), l2.is_none(), carry);
            if l1.is_none() && l2.is_none() && carry == 0 {
                break;
            }
        }
        ans.next
    }
}
