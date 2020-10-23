use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_palindrome(None));
}
#[test]
fn case_2() {
    assert!(Solution::is_palindrome(ListNode::from_array(&[1])));
}
#[test]
fn case_3() {
    assert!(!Solution::is_palindrome(ListNode::from_array(&[1, 2])));
}
#[test]
fn case_4() {
    assert!(!Solution::is_palindrome(ListNode::from_array(&[1, 2, 3])));
}

#[test]
fn case_5() {
    assert!(Solution::is_palindrome(ListNode::from_array(&[1, 2, 2, 1])));
}

#[test]
fn case_6() {
    assert!(Solution::is_palindrome(ListNode::from_array(&[
        1, 2, 3, 2, 1
    ])));
}
