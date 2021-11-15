use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::friend_requests(3, vv![[0, 1]], vv![[0, 2], [2, 1]]),
        vv![true, false]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::friend_requests(3, vv![[0, 1]], vv![[1, 2], [0, 2]]),
        vv![true, false]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::friend_requests(
            5,
            vv![[0, 1], [1, 2], [2, 3]],
            vv![[0, 4], [1, 2], [3, 1], [3, 4]]
        ),
        vv![true, false, true, false]
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::friend_requests(
            8,
            vv![
                [6, 4],
                [7, 5],
                [2, 6],
                [1, 5],
                [6, 7],
                [6, 5],
                [0, 3],
                [5, 4],
                [0, 4],
                [2, 7],
                [0, 2]
            ],
            vv![
                [6, 3],
                [0, 2],
                [0, 5],
                [0, 3],
                [6, 4],
                [2, 4],
                [1, 0],
                [2, 1],
                [2, 5],
                [6, 7],
                [7, 0],
                [3, 2],
                [3, 5],
                [2, 1],
                [1, 6],
                [7, 4],
                [6, 3],
                [1, 3],
                [6, 5],
                [3, 7],
                [7, 0],
                [6, 5],
                [0, 5],
                [0, 4],
                [7, 5],
                [7, 0],
                [7, 0],
                [1, 3]
            ]
        ),
        vv![
            true, false, true, false, false, true, false, true, false, false, false, false, false,
            true, false, false, true, false, false, false, false, false, true, false, false, false,
            false, false
        ]
    );
}
