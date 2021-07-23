use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::maximum_time("2?:?0".to_string()),
        "23:50".to_string()
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::maximum_time("0?:3?".to_string()),
        "09:39".to_string()
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::maximum_time("1?:22".to_string()),
        "19:22".to_string()
    );
}
