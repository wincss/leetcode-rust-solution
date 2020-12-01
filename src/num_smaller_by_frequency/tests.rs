use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::num_smaller_by_frequency(to_string_vec(&["cbd"]), to_string_vec(&["zaaaz"])),
        vec![1]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::num_smaller_by_frequency(
            to_string_vec(&["bbb", "cc"]),
            to_string_vec(&["a", "aa", "aaa", "aaaa"])
        ),
        vec![1, 2]
    );
}
