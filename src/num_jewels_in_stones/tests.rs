use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        3
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()),
        0
    );
}
