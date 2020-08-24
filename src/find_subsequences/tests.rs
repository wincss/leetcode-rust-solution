use crate::*;

use std::collections::HashSet;
#[test]
fn case_1() {
    assert_eq!(
        Solution::find_subsequences(vec![4, 6, 7, 7])
            .into_iter()
            .collect::<HashSet<Vec<i32>>>(),
        vec![
            vec![4, 6],
            vec![4, 7],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7],
            vec![4, 7, 7]
        ]
        .into_iter()
        .collect()
    )
}

// TODO: add more tests
