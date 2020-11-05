use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::ladder_length(
            String::from("hit"),
            String::from("cog"),
            to_string_vec(&["hot", "dot", "dog", "lot", "log", "cog"])
        ),
        5
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::ladder_length(
            String::from("hit"),
            String::from("cog"),
            to_string_vec(&["hot", "dot", "dog", "lot", "log"])
        ),
        0
    );
}
