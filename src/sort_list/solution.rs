use crate::*;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn clone_mut_ref<T>(p: &mut T) -> (&mut T, &mut T) {
            unsafe { (&mut *(p as *mut T), &mut *(p as *mut T)) }
        }
        let mut head = head;
        let (mut slow, mut fast) = clone_mut_ref(&mut head);
        let mut slow_moved = false;
        while fast.is_some() {
            fast = &mut fast.as_mut().unwrap().next;
            if fast.is_none() {
                break;
            }
            fast = &mut fast.as_mut().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
            slow_moved = true;
        }
        if !slow_moved {
            return head;
        }
        let mut right = Solution::sort_list(slow.take());
        let mut left = Solution::sort_list(head);
        let mut result = None;
        let mut tail = &mut result;
        while left.is_some() || right.is_some() {
            if left.is_none() {
                tail.replace(right.unwrap());
                break;
            }
            if right.is_none() {
                tail.replace(left.unwrap());
                break;
            }
            let mut v1 = left.unwrap();
            let mut v2 = right.unwrap();
            if v1.val < v2.val {
                left = v1.next.take();
                right = Some(v2);
                tail.replace(v1);
            } else {
                right = v2.next.take();
                left = Some(v1);
                tail.replace(v2);
            }
            tail = &mut tail.as_mut().unwrap().next;
        }
        result
    }
}
