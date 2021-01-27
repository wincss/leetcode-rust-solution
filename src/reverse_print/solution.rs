use crate::*;

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut head = Self::reverse_list(head);
        let mut result = vec![];
        while let Some(v) = head.take() {
            result.push(v.val);
            head = v.next;
        }
        result
    }
}
