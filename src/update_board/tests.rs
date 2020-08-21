use crate::*;

#[test]
fn case_1() {
    let input = vec![
        vec!['E', 'E', 'E', 'E', 'E'],
        vec!['E', 'E', 'M', 'E', 'E'],
        vec!['E', 'E', 'E', 'E', 'E'],
        vec!['E', 'E', 'E', 'E', 'E'],
    ];
    let click = vec![3, 0];
    assert_eq!(
        Solution::update_board(input, click),
        vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'M', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B']
        ]
    );
}

#[test]
fn case_2() {
    let input = vec![
        vec!['B', '1', 'E', '1', 'B'],
        vec!['B', '1', 'M', '1', 'B'],
        vec!['B', '1', '1', '1', 'B'],
        vec!['B', 'B', 'B', 'B', 'B'],
    ];
    let click = vec![1, 2];
    assert_eq!(
        Solution::update_board(input, click),
        vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'X', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B']
        ]
    );
}

#[test]
fn case_3() {
    let input = vec![vec!['E'; 50]; 50];
    let click = vec![29, 2];
    assert_eq!(
        Solution::update_board(input, click),
        vec![vec!['B'; 50]; 50]
    );
}
