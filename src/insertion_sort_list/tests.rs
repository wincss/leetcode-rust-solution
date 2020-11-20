use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::insertion_sort_list(None), None);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::insertion_sort_list(ListNode::from_array(&[1])),
        ListNode::from_array(&[1])
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::insertion_sort_list(ListNode::from_array(&[1, 3, 2])),
        ListNode::from_array(&[1, 2, 3])
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::insertion_sort_list(ListNode::from_array(&[4, 2, 1, 3])),
        ListNode::from_array(&[1, 2, 3, 4])
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::insertion_sort_list(ListNode::from_array(&[-1, 5, 3, 4, 0])),
        ListNode::from_array(&[-1, 0, 3, 4, 5])
    );
}
