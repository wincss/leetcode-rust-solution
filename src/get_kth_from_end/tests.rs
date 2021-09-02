use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::get_kth_from_end(ListNode::from_array(&[1, 2, 3, 4, 5]), 2),
        ListNode::from_array(&[4, 5])
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::get_kth_from_end(ListNode::from_array(&[1, 2, 3, 4, 5]), 1),
        ListNode::from_array(&[5])
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::get_kth_from_end(ListNode::from_array(&[1, 2, 3, 4, 5]), 5),
        ListNode::from_array(&[1, 2, 3, 4, 5])
    );
}
