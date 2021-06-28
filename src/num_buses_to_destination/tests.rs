use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::num_buses_to_destination(
            vec![
                vec![7, 12],
                vec![4, 5, 15],
                vec![6],
                vec![15, 19],
                vec![9, 12, 13]
            ],
            15,
            12
        ),
        -1
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::num_buses_to_destination(vec![vec![1, 7], vec![3, 5]], 5, 5),
        0
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::num_buses_to_destination(
            vec![vec![1], vec![15, 16, 18], vec![10], vec![3, 4, 12, 14]],
            3,
            15
        ),
        -1
    );
}
