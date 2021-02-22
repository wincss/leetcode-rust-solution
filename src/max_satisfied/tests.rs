use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::max_satisfied(
            vec![1, 0, 1, 2, 1, 1, 7, 5],
            vec![0, 1, 0, 1, 0, 1, 0, 1],
            3
        ),
        16
    );
}
