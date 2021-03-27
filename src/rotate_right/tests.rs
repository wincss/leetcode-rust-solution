use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::rotate_right(ListNode::from_array(&[1, 2, 3, 4, 5]), 2),
        ListNode::from_array(&[4, 5, 1, 2, 3])
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::rotate_right(ListNode::from_array(&[0, 1, 2]), 4),
        ListNode::from_array(&[2, 0, 1])
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::rotate_right(ListNode::from_array(&[]), 0),
        ListNode::from_array(&[])
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::rotate_right(ListNode::from_array(&[]), 1),
        ListNode::from_array(&[])
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::rotate_right(ListNode::from_array(&[1]), 0),
        ListNode::from_array(&[1])
    );
}

#[test]
fn case_6() {
    assert_eq!(
        Solution::rotate_right(ListNode::from_array(&[1]), 1),
        ListNode::from_array(&[1])
    );
}

#[test]
fn case_7() {
    assert_eq!(
        Solution::rotate_right(ListNode::from_array(&[1, 2]), 0),
        ListNode::from_array(&[1, 2])
    );
}
