use crate::*;

#[test]
fn case_1() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    assert!(Solution::exist(board, String::from("ABCCED")));
}

#[test]
fn case_2() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    assert!(Solution::exist(board, String::from("SEE")));
}

#[test]
fn case_3() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    assert!(!Solution::exist(board, String::from("ABCB")));
}

#[test]
fn case_4() {
    let board = vec![vec!['a']];
    assert!(Solution::exist(board, String::from("a")));
}
