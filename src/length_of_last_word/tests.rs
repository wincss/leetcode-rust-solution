use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::length_of_last_word(s!("Hello World")), 5);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::length_of_last_word(s!("   fly me   to   the moon  ")),
        4
    );
}
#[test]
fn case_3() {
    assert_eq!(
        Solution::length_of_last_word(s!("luffy is still joyboy")),
        6
    );
}
