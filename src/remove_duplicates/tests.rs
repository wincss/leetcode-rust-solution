use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::remove_duplicates(String::from("abbaca")),
        String::from("ca")
    );
}
