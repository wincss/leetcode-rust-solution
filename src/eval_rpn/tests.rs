use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::eval_rpn(to_string_vec(&["2", "1", "+", "3", "*"])),
        9
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::eval_rpn(to_string_vec(&["4", "13", "5", "/", "+"])),
        6
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::eval_rpn(to_string_vec(&[
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
        ])),
        22
    );
}
