use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::remove_elements(ListNode::from_array(&[1, 2, 6, 3, 4, 5, 6]), 6),
        ListNode::from_array(&[1, 2, 3, 4, 5])
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::remove_elements(ListNode::from_array(&[]), 1),
        ListNode::from_array(&[])
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::remove_elements(ListNode::from_array(&[7, 7, 7, 7]), 7),
        ListNode::from_array(&[])
    );
}
