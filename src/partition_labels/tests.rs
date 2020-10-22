use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::partition_labels(String::from("ababcbacadefegdehijhklij")),
        vec![9, 7, 8]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::partition_labels(String::from("abcdefg")),
        vec![1, 1, 1, 1, 1, 1, 1]
    );
}
