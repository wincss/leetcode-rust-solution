use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::max_product_2002(s!("leetcodecom")), 9);
}

#[test]
fn case_2() {
    assert_eq!(Solution::max_product_2002(s!("bb")), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::max_product_2002(s!("accbcaxxcxx")), 25);
}

#[test]
fn case_318_1() {
    assert_eq!(
        Solution::max_product_318(sv!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]),
        16
    );
}

#[test]
fn case_318_2() {
    assert_eq!(
        Solution::max_product_318(sv!["a", "ab", "abc", "d", "cd", "bcd", "abcd"]),
        4
    );
}

#[test]
fn case_318_3() {
    assert_eq!(Solution::max_product_318(sv!["a", "aa", "aaa", "aaaa"]), 0);
}
