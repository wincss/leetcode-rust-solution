use crate::*;

#[test]
fn case_1() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    Solution::solve(&mut board);
    assert_eq!(
        board,
        vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ]
    )
}

#[test]
fn case_2() {
    let mut board = vec![vec!['O']];
    Solution::solve(&mut board);
    assert_eq!(board, vec![vec!['O'],])
}
