use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
        3
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
        -1
    );
}
