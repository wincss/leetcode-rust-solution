use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::max_height(vec![vec![50, 45, 20], vec![95, 37, 53], vec![45, 23, 12]]),
        190
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::max_height(vec![vec![38, 25, 45], vec![76, 35, 3]]),
        76
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::max_height(vec![
            vec![7, 11, 17],
            vec![7, 17, 11],
            vec![11, 7, 17],
            vec![11, 17, 7],
            vec![17, 7, 11],
            vec![17, 11, 7]
        ]),
        102
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::max_height(vec![vec![35, 32, 11], vec![7, 6, 65], vec![3, 39, 41]]),
        65
    );
}
