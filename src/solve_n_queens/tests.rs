use crate::*;

use std::collections::HashSet;
#[test]
fn case_1() {
    let result = Solution::solve_n_queens(4)
        .into_iter()
        .collect::<HashSet<Vec<String>>>();
    let answer = vec![
        to_string_vec(&[".Q..", "...Q", "Q...", "..Q."]),
        to_string_vec(&["..Q.", "Q...", "...Q", ".Q.."]),
    ]
    .into_iter()
    .collect::<HashSet<Vec<String>>>();
    assert_eq!(result, answer);
}
