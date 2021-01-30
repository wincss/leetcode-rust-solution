use crate::*;

#[test]
fn case_6() {
    assert_eq!(Solution::swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::swim_in_water(vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6]
        ]),
        16
    );
}
