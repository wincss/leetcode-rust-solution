use crate::*;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        fn clone_mut_ref<T>(p: &mut T) -> (&mut T, &mut T) {
            unsafe { (&mut *(p as *mut T), &mut *(p as *mut T)) }
        }
        let mut root = ListNode::new(0);
        root.next = head.take();
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
        let mut current = &mut root.as_mut().unwrap().next;
        while last.is_some() {
            let primary = current.as_mut().unwrap().next.take();
            let secondary = last.as_mut().unwrap().next.take();
            last.as_mut().unwrap().next = primary;
            current.as_mut().unwrap().next = last;
            last = secondary;
            current = &mut current.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        if root.as_ref().unwrap().next.is_some() {
            head.replace(root.as_mut().unwrap().next.take().unwrap());
        }
    }
}
