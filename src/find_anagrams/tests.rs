use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::find_anagrams(String::from("cbaebabacd"), String::from("abc"));
    output.sort();
    assert_eq!(output, [0, 6]);
}

#[test]
fn case_2() {
    let mut output = Solution::find_anagrams(String::from("abab"), String::from("ab"));
    output.sort();
    assert_eq!(output, [0, 1, 2]);
}
