use crate::*;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        fn clone_mut_ref<T>(p: &mut T) -> (&mut T, &mut T) {
            unsafe { (&mut *(p as *mut T), &mut *(p as *mut T)) }
        }
        let mut root = ListNode::new(0);
        root.next = head;
        let mut root = Some(Box::new(root));
        let (mut slow, root) = clone_mut_ref(&mut root);
        let (mut fast, root) = clone_mut_ref(root);
        while fast.is_some() {
            fast = &mut fast.as_mut().unwrap().next;
            if fast.is_none() {
                break;
            }
            fast = &mut fast.as_mut().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }
        let mut tail = slow.as_mut().unwrap().next.take();
        let mut last = None;
        while tail.is_some() {
            let next = tail.as_mut().unwrap().next.take();
            tail.as_mut().unwrap().next = last;
            last = tail;
            tail = next;
        }
        let mut root = root.as_mut().unwrap().next.take();
        while root.is_some() && last.is_some() {
            if root.as_ref().unwrap().val != last.as_ref().unwrap().val {
                return false;
            }
            root = root.as_mut().unwrap().next.take();
            last = last.as_mut().unwrap().next.take();
        }
        true
    }
}
