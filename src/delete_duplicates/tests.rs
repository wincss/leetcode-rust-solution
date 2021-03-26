use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::delete_duplicates_ii(ListNode::from_array(&[1, 2, 3, 3, 4, 4, 5])),
        ListNode::from_array(&[1, 2, 5])
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::delete_duplicates_ii(ListNode::from_array(&[1, 1, 1, 2, 3])),
        ListNode::from_array(&[2, 3])
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_array(&[1, 1, 2])),
        ListNode::from_array(&[1, 2])
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::delete_duplicates(ListNode::from_array(&[1, 1, 2, 3, 3])),
        ListNode::from_array(&[1, 2, 3])
    );
}
