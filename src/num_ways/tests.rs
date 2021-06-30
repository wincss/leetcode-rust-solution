use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::num_ways_1269(3, 2), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::num_ways_1269(2, 4), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::num_ways_1269(4, 2), 8);
}

#[test]
fn case_4() {
    assert_eq!(Solution::num_ways_1269(27, 7), 127784505);
}

#[test]
fn case_lcp_07_1() {
    assert_eq!(
        Solution::num_ways_lcp_07(
            5,
            vec![
                vec![0, 2],
                vec![2, 1],
                vec![3, 4],
                vec![2, 3],
                vec![1, 4],
                vec![2, 0],
                vec![0, 4]
            ],
            3
        ),
        3
    );
}

#[test]
fn case_lcp_07_2() {
    assert_eq!(
        Solution::num_ways_lcp_07(3, vec![vec![0, 2], vec![2, 1],], 2),
        0
    );
}
