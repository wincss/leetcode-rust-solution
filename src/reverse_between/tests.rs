use crate::*;

#[test]
fn case_1() {
    let input = ListNode::from_array(&[1, 2, 3, 4, 5]);
    let output = Solution::reverse_between(input, 2, 4);
    assert_eq!(output.unwrap().to_vec(), vec![1, 4, 3, 2, 5]);
}

#[test]
fn case_2() {
    let input = ListNode::from_array(&[5]);
    let output = Solution::reverse_between(input, 1, 1);
    assert_eq!(output.unwrap().to_vec(), vec![5]);
}
