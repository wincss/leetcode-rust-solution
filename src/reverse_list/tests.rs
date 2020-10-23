use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::reverse_list(ListNode::from_array(&[1, 2, 3, 4, 5])),
        ListNode::from_array(&[5, 4, 3, 2, 1])
    )
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::reverse_list(ListNode::from_array(&[1])),
        ListNode::from_array(&[1])
    )
}

#[test]
fn case_3() {
    assert_eq!(Solution::reverse_list(None), None)
}
