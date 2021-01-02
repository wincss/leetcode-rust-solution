use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::partition(ListNode::from_array(&[1, 4, 3, 2, 5, 2]), 3),
        ListNode::from_array(&[1, 2, 2, 4, 3, 5])
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::partition(ListNode::from_array(&[1, 4, 3, 2, 5, 2]), 100),
        ListNode::from_array(&[1, 4, 3, 2, 5, 2])
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::partition(ListNode::from_array(&[1, 4, 3, 2, 5, 2]), 0),
        ListNode::from_array(&[1, 4, 3, 2, 5, 2])
    );
}

#[test]
fn case_4() {
    assert_eq!(Solution::partition(None, 0), None);
}
