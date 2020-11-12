use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::odd_even_list(ListNode::from_array(&[1, 2, 3, 4, 5])),
        ListNode::from_array(&[1, 3, 5, 2, 4])
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::odd_even_list(ListNode::from_array(&[2, 1, 3, 5, 6, 4, 7])),
        ListNode::from_array(&[2, 3, 6, 7, 1, 5, 4])
    );
}

#[test]
fn case_3() {
    assert_eq!(Solution::odd_even_list(None), None);
}
