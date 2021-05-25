use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::reverse_parentheses("(abcd)".to_string()),
        "dcba".to_string()
    );
}
#[test]
fn case_2() {
    assert_eq!(
        Solution::reverse_parentheses("(u(love)i)".to_string()),
        "iloveu".to_string()
    );
}
#[test]
fn case_3() {
    assert_eq!(
        Solution::reverse_parentheses("(ed(et(oc))el)".to_string()),
        "leetcode".to_string()
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::reverse_parentheses("a(bcdefghijkl(mno)p)q".to_string()),
        "apmnolkjihgfedcbq".to_string()
    );
}
