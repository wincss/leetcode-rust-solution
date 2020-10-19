use crate::*;

#[test]
fn case_1() {
    let mut head = ListNode::from_array(&[]);
    Solution::reorder_list(&mut head);
    assert_eq!(head, ListNode::from_array(&[]));
}

#[test]
fn case_2() {
    let mut head = ListNode::from_array(&[1, 2, 3, 4]);
    Solution::reorder_list(&mut head);
    assert_eq!(head, ListNode::from_array(&[1, 4, 2, 3]));
}

#[test]
fn case_3() {
    let mut head = ListNode::from_array(&[1, 2, 3, 4, 5]);
    Solution::reorder_list(&mut head);
    assert_eq!(head, ListNode::from_array(&[1, 5, 2, 4, 3]));
}
