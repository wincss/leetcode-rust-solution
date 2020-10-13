use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::common_chars(to_string_vec(&["bella", "label", "roller"]));
    output.sort();
    assert_eq!(output, to_string_vec(&["e", "l", "l"]));
}

#[test]
fn case_2() {
    let mut output = Solution::common_chars(to_string_vec(&["cool", "lock", "cook"]));
    output.sort();
    assert_eq!(output, to_string_vec(&["c", "o"]));
}
