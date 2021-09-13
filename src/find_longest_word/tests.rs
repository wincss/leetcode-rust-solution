use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_longest_word(s!("abpcplea"), sv!["ale", "apple", "monkey", "plea"]),
        s!("apple")
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_longest_word(s!("abpcplea"), sv!["a", "b", "c"]),
        s!("a")
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::find_longest_word(s!("abce"), sv!["abe", "abc"]),
        s!("abc")
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::find_longest_word(
            s!("abpcplea"),
            sv![
                "ale",
                "apple",
                "monkey",
                "plea",
                "abpcplaaa",
                "abpcllllll",
                "abccclllpppeeaaaa"
            ]
        ),
        s!("apple")
    );
}
