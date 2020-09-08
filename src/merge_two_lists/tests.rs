use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::merge_two_lists(
            ListNode::from_array(&[1, 2, 4]),
            ListNode::from_array(&[1, 3, 4]),
        ),
        ListNode::from_array(&[1, 1, 2, 3, 4, 4]),
    )
}
