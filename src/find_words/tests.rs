use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::find_words(
        to_board([
            ["o", "a", "a", "n"],
            ["e", "t", "a", "e"],
            ["i", "h", "k", "r"],
            ["i", "f", "l", "v"],
        ]),
        sv!["oath", "pea", "eat", "rain"],
    );
    output.sort();
    assert_eq!(output, sv!["eat", "oath"]);
}

#[test]
fn case_2() {
    let output = Solution::find_words(to_board([["a", "b"], ["c", "d"]]), sv!["abcd"]);
    assert_eq!(output, sv![]);
}

#[test]
fn case_3() {
    let mut output = Solution::find_words(
        to_board([["a", "b", "c"], ["a", "e", "d"], ["a", "f", "g"]]),
        sv!["abcdefg", "gfedcbaaa", "eaabcdgfa", "befa", "dgc", "ade"],
    );
    output.sort();
    assert_eq!(output, sv!["abcdefg", "befa", "eaabcdgfa", "gfedcbaaa"]);
}

#[test]
fn case_4() {
    let mut output = Solution::find_words(
        to_board([
            ["o", "a", "b", "n"],
            ["o", "t", "a", "e"],
            ["a", "h", "k", "r"],
            ["a", "f", "l", "v"],
        ]),
        sv!["oa", "oaa"],
    );
    output.sort();
    assert_eq!(output, sv!["oa", "oaa"]);
}
