use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::connect_two_groups(vec![vec![15, 96], vec![36, 2]]),
        17
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::connect_two_groups(vec![vec![1, 3, 5], vec![4, 1, 1], vec![1, 5, 3]]),
        4
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::connect_two_groups(vec![
            vec![2, 5, 1],
            vec![3, 4, 7],
            vec![8, 1, 2],
            vec![6, 2, 4],
            vec![3, 8, 8]
        ]),
        10
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::connect_two_groups(vec![
            vec![93, 56, 92],
            vec![53, 44, 18],
            vec![86, 44, 69],
            vec![54, 60, 30]
        ]),
        172
    );
}
