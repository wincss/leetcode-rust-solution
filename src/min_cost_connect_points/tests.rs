use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::min_cost_connect_points(vec![
            vec![0, 0],
            vec![2, 2],
            vec![3, 10],
            vec![5, 2],
            vec![7, 0]
        ]),
        20
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::min_cost_connect_points(vec![vec![3, 12], vec![-2, 5], vec![-4, 1]]),
        18
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![-1, 1]]),
        4
    );
}
#[test]
fn case_4() {
    assert_eq!(
        Solution::min_cost_connect_points(vec![vec![-1000000, -1000000], vec![1000000, 1000000]]),
        4000000
    );
}
#[test]
fn case_5() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0]]), 0);
}
#[test]
fn case_6() {
    assert_eq!(Solution::min_cost_connect_points(vec![]), 0);
}
