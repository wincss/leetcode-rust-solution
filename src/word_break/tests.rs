use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::word_break(
        String::from("catsanddog"),
        to_string_vec(&["cat", "cats", "and", "sand", "dog"]),
    );
    output.sort();
    assert_eq!(output, to_string_vec(&["cat sand dog", "cats and dog"]));
}

#[test]
fn case_2() {
    let mut output = Solution::word_break(
        String::from("pineapplepenapple"),
        to_string_vec(&["apple", "pen", "applepen", "pine", "pineapple"]),
    );
    output.sort();
    assert_eq!(
        output,
        to_string_vec(&[
            "pine apple pen apple",
            "pine applepen apple",
            "pineapple pen apple",
        ])
    );
}

#[test]
fn case_3() {
    let mut output = Solution::word_break(
        String::from("catsandog"),
        to_string_vec(&["cats", "dog", "sand", "and", "cat"]),
    );
    output.sort();
    assert_eq!(output, Vec::<String>::new());
}
