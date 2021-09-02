use crate::*;

impl Solution {
    pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn clone_mut_ref<T>(p: &mut T) -> (&mut T, &mut T) {
            unsafe { (&mut *(p as *mut T), &mut *(p as *mut T)) }
        }
        let mut head = head;
        let (mut slow, mut fast) = clone_mut_ref(&mut head);
        for _ in 0..k {
            fast = &mut fast.as_mut().unwrap().next;
        }
        while fast.is_some() {
            slow = &mut slow.as_mut().unwrap().next;
            fast = &mut fast.as_mut().unwrap().next;
        }
        slow.take()
    }
}
