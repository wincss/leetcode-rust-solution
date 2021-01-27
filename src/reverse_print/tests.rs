use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::reverse_print(ListNode::from_array(&[1, 3, 2])),
        vec![2, 3, 1]
    );
}
