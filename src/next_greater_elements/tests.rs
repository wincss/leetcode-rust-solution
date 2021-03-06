use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::next_greater_elements(vec![1, 2, 1]),
        vec![2, -1, 2]
    );
}
