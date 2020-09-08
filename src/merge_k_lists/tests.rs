use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::merge_k_lists(vec![
            ListNode::from_array(&[1, 4, 5]),
            ListNode::from_array(&[1, 3, 4]),
            ListNode::from_array(&[2, 6]),
        ]),
        ListNode::from_array(&[1, 1, 2, 3, 4, 4, 5, 6]),
    )
}

#[test]
fn case_2() {
    assert_eq!(Solution::merge_k_lists(vec![]), None)
}

#[test]
fn case_3() {
    assert_eq!(Solution::merge_k_lists(vec![None]), None)
}
