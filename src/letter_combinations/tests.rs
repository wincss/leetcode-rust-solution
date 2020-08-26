use crate::*;

use std::collections::HashSet;

#[test]
fn case_1() {
    let result = to_string_vec(&["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"])
        .into_iter()
        .collect::<HashSet<String>>();
    assert_eq!(
        Solution::letter_combinations(String::from("23"))
            .into_iter()
            .collect::<HashSet<String>>(),
        result
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::letter_combinations(String::new()),
        Vec::<String>::new()
    );
}
