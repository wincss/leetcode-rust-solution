use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
        8
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0),
        6
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::least_interval(
            vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
            2
        ),
        16
    );
}
