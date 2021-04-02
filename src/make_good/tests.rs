use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::make_good("leEeetcode".to_string()),
        "leetcode".to_string()
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::make_good("abBAcC".to_string()), "".to_string());
}

#[test]
fn case_3() {
    assert_eq!(Solution::make_good("s".to_string()), "s".to_string());
}
