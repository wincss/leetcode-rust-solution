use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_lu_slength("aba".to_string(), "cdc".to_string()),
        3
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_lu_slength("aaa".to_string(), "bbb".to_string()),
        3
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::find_lu_slength("aaa".to_string(), "aaa".to_string()),
        -1
    );
}
