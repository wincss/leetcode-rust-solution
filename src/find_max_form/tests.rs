use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_max_form(to_string_vec(&["10", "0001", "111001", "1", "0"]), 5, 3),
        4
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_max_form(to_string_vec(&["10", "0", "1", "1", "0"]), 1, 1),
        2
    );
}
