use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::smallest_string_with_swaps(String::from("dcab"), vec![vec![0, 3], vec![1, 2]]),
        String::from("bacd")
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::smallest_string_with_swaps(
            String::from("dcab"),
            vec![vec![0, 3], vec![1, 2], vec![0, 2]]
        ),
        String::from("abcd")
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::smallest_string_with_swaps(String::from("cba"), vec![vec![0, 1], vec![1, 2]]),
        String::from("abc")
    );
}
