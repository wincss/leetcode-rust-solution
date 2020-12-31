use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::maximize_xor(
            vec![0, 1, 2, 3, 4],
            vec![vec![3, 1], vec![1, 3], vec![5, 6]]
        ),
        vec![3, 3, 7]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::maximize_xor(
            vec![5, 2, 4, 6, 6, 3],
            vec![vec![12, 4], vec![8, 1], vec![6, 3]]
        ),
        vec![15, -1, 5]
    );
}
