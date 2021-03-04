use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
        3
    );
}
