use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::split_list_to_parts(ListNode::from_array(&[1, 2, 3]), 5),
        vec![
            ListNode::from_array(&[1]),
            ListNode::from_array(&[2]),
            ListNode::from_array(&[3]),
            None,
            None
        ]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::split_list_to_parts(ListNode::from_array(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 3),
        vec![
            ListNode::from_array(&[1, 2, 3, 4]),
            ListNode::from_array(&[5, 6, 7]),
            ListNode::from_array(&[8, 9, 10]),
        ]
    );
}
