use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_valid_serialization(
        "9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()
    ))
}

#[test]
fn case_2() {
    assert!(Solution::is_valid_serialization("#".to_string()))
}
#[test]
fn case_3() {
    assert!(!Solution::is_valid_serialization("".to_string()))
}
#[test]
fn case_4() {
    assert!(!Solution::is_valid_serialization("1,#".to_string()))
}
#[test]
fn case_5() {
    assert!(!Solution::is_valid_serialization("9,#,#,1".to_string()))
}
#[test]
fn case_6() {
    assert!(Solution::is_valid_serialization("9,#,92,#,#".to_string()))
}
#[test]
fn case_7() {
    assert!(!Solution::is_valid_serialization("#,#,3,5,#".to_string()))
}
#[test]
fn case_8() {
    assert!(!Solution::is_valid_serialization(
        "#,7,6,9,#,#,#".to_string()
    ))
}
