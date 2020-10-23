use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::remove_nth_from_end(ListNode::from_array(&[1, 2, 3, 4, 5]), 2),
        ListNode::from_array(&[1, 2, 3, 5])
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::remove_nth_from_end(ListNode::from_array(&[1, 2]), 2),
        ListNode::from_array(&[2])
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::remove_nth_from_end(ListNode::from_array(&[1]), 1),
        None
    );
}
