use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::palindrome_partition("aab".to_string());
    output.sort();
    assert_eq!(
        output,
        vec![to_string_vec(&["a", "a", "b"]), to_string_vec(&["aa", "b"])]
    )
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::palindrome_partition("a".to_string()),
        vec![vec!["a".to_string()]]
    )
}
