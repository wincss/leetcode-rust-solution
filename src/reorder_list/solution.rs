use crate::*;

use std::collections::VecDeque;
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        let mut nodes = VecDeque::new();
        let mut this = head.as_mut().unwrap().next.take();
        while this.is_some() {
            let next = this.as_mut().unwrap().next.take();
            nodes.push_back(this.unwrap());
            this = next;
        }
        let mut direct = true;
        let mut this = head;
        while !nodes.is_empty() {
            let next = if direct {
                nodes.pop_back()
            } else {
                nodes.pop_front()
            };
            direct = !direct;
            this.as_mut().unwrap().next.replace(next.unwrap());
            this = &mut this.as_mut().unwrap().next;
        }
    }
}
