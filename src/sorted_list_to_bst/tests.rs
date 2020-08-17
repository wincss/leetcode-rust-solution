use crate::*;

use std::rc::Rc;

fn check_answer(nums: &[i32]) {
    let input = ListNode::from_array(&nums);
    let check_none = input.is_none();
    let output = Solution::sorted_list_to_bst(input);
    if check_none {
        assert!(output.is_none());
    } else {
        assert!(output.is_some());
        assert_eq!(
            &Rc::clone(output.as_ref().unwrap()).borrow().inorder()[..],
            nums
        );
        assert!(Solution::is_balanced(output));
    }
}

#[test]
fn case_1() {
    check_answer(&[-10, -3, 0, 5, 9]);
}

#[test]
fn case_2() {
    check_answer(&[]);
}
