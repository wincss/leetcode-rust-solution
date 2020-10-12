use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::swap_pairs(ListNode::from_array(&[1, 2, 3, 4])),
        ListNode::from_array(&[2, 1, 4, 3])
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::swap_pairs(ListNode::from_array(&[])),
        ListNode::from_array(&[])
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::swap_pairs(ListNode::from_array(&[1, 2, 3])),
        ListNode::from_array(&[2, 1, 3])
    );
}
