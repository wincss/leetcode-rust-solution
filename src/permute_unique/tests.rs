use crate::*;

use std::collections::HashSet;

fn check_result(input: Vec<i32>) {
    let mut input = input;
    let output = Solution::permute_unique(input.clone());
    let output_len = output.len();
    let unique = output.into_iter().collect::<HashSet<Vec<i32>>>();
    // check uniqueness
    assert_eq!(output_len, unique.len());
    // check permutation number
    input.sort();
    let mut permutation_number = 1;
    let mut last_number = 0;
    let mut last_count = 0;
    for (i, &v) in input.iter().enumerate() {
        permutation_number *= i + 1;
        if v == last_number {
            last_count += 1;
        } else {
            last_number = v;
            last_count = 1;
        }
        permutation_number /= last_count;
    }
    assert_eq!(output_len, permutation_number);
    // check every permutation match input
    for mut permutation in unique {
        permutation.sort();
        assert_eq!(permutation, input);
    }
}

#[test]
fn case_1() {
    check_result(vec![1, 2, 1]);
}

#[test]
fn case_2() {
    check_result(vec![1, 2, 3]);
}
