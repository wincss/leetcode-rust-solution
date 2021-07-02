use crate::*;

use std::collections::HashMap;
fn check_result(input: String) {
    let mut ch: Vec<_> = input.chars().collect();
    ch.sort();
    let mut h = HashMap::new();
    for &c in ch.iter() {
        *h.entry(c).or_insert(0_usize) += 1;
    }
    let output = Solution::frequency_sort(input);
    let mut ch2: Vec<_> = output.chars().collect();
    let mut last = std::usize::MAX;
    for c in ch2.iter() {
        assert!(h[c] <= last);
        last = h[c];
    }
    ch2.sort();
    assert_eq!(ch, ch2);
    // "cacaca" is incorrect but no rule for it
}

#[test]
fn case_1() {
    check_result("tree".to_string());
}

#[test]
fn case_2() {
    check_result("cccaaa".to_string());
}

#[test]
fn case_3() {
    check_result("Aabb".to_string());
}
