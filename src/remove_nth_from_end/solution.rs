use crate::*;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        fn clone_mut_ref<T>(p: &mut T) -> (&mut T, &mut T) {
            unsafe { (&mut *(p as *mut T), &mut *(p as *mut T)) }
        }

        let mut root = ListNode::new(0);
        root.next = head;
        let mut root = Some(Box::new(root));
        let (mut slow, mut fast) = clone_mut_ref(&mut root);

        for _ in 0..=n {
            fast = &mut fast.as_mut().unwrap().next;
        }
        while fast.is_some() {
            slow = &mut slow.as_mut().unwrap().next;
            fast = &mut fast.as_mut().unwrap().next;
        }
        let mut to_delete = slow.as_mut().unwrap().next.take();
        slow.as_mut().unwrap().next = to_delete.as_mut().unwrap().next.take();
        root.unwrap().next
    }
}
