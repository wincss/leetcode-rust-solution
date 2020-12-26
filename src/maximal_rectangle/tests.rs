use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::maximal_rectangle(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ]),
        6
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::maximal_rectangle(vec![]), 0);
}

#[test]
fn case_3() {
    assert_eq!(Solution::maximal_rectangle(vec![vec!['0']]), 0);
}

#[test]
fn case_4() {
    assert_eq!(Solution::maximal_rectangle(vec![vec!['1']]), 1);
}

#[test]
fn case_5() {
    assert_eq!(Solution::maximal_rectangle(vec![vec!['0', '0']]), 0);
}
