use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
        1
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
        2
    );
}
