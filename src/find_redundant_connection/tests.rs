use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
        vec![2, 3]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_redundant_connection(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![1, 4],
            vec![1, 5]
        ]),
        vec![1, 4]
    );
}
