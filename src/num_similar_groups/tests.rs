use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::num_similar_groups(to_string_vec(&["tars", "rats", "arts", "star"])),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::num_similar_groups(to_string_vec(&["omv", "ovm"])),
        1
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::num_similar_groups(to_string_vec(&["abc", "abc"])),
        1
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::num_similar_groups(to_string_vec(&["abc", "abc", "abc"])),
        1
    );
}
