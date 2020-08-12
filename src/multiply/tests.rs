use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::multiply("2".to_string(), "3".to_string()),
        "6".to_string()
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::multiply("123".to_string(), "456".to_string()),
        "56088".to_string()
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::multiply("0".to_string(), "456".to_string()),
        "0".to_string()
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::multiply("4999".to_string(), "9999".to_string()),
        "49985001".to_string()
    );
}
