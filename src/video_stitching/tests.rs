use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::video_stitching(
            vec![
                vec![0, 2],
                vec![4, 6],
                vec![8, 10],
                vec![1, 9],
                vec![1, 5],
                vec![5, 9]
            ],
            10
        ),
        3
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::video_stitching(vec![vec![0, 1], vec![1, 2]], 5),
        -1
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::video_stitching(
            vec![
                vec![0, 1],
                vec![6, 8],
                vec![0, 2],
                vec![5, 6],
                vec![0, 4],
                vec![0, 3],
                vec![6, 7],
                vec![1, 3],
                vec![4, 7],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6],
                vec![3, 4],
                vec![4, 5],
                vec![5, 7],
                vec![6, 9]
            ],
            9
        ),
        3
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::video_stitching(vec![vec![0, 4], vec![2, 8]], 5),
        2
    );
}
