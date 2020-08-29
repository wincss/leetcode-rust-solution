use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::reverse_words(String::from("Let's take LeetCode contest")),
        String::from("s'teL ekat edoCteeL tsetnoc")
    )
}
