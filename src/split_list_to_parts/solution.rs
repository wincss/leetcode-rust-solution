use crate::*;

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut count = 0;
        let mut current = &head;
        while let Some(this) = current.as_ref() {
            count += 1;
            current = &this.next;
        }
        let part = count / k;
        let mut remainder = count % k;

        let mut result = vec![];
        let mut head = head;
        for _ in 0..k {
            result.push(head);
            let part_length = part
                + (if remainder > 0 {
                    remainder -= 1;
                    1
                } else {
                    0
                });
            if part_length == 0 {
                head = result.last_mut().unwrap().take();
            } else {
                let mut last = result.last_mut().unwrap();
                for _ in 1..part_length {
                    last = &mut last.as_mut().unwrap().next;
                }
                head = last.as_mut().unwrap().next.take();
            }
        }
        result
    }
}
