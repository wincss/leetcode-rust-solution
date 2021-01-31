use crate::*;

use std::collections::HashSet;

fn check_answer(a: Vec<i32>, b: Vec<i32>) {
    let sum_a: i32 = a.iter().sum();
    let sum_b: i32 = b.iter().sum();
    let ha: HashSet<i32> = a.clone().into_iter().collect();
    let hb: HashSet<i32> = b.clone().into_iter().collect();
    let output = Solution::fair_candy_swap(a, b);
    println!("{:?}", output);
    assert_eq!(output.len(), 2);
    assert!(ha.contains(&output[0]));
    assert!(hb.contains(&output[1]));
    assert_eq!(sum_a - output[0] + output[1], sum_b - output[1] + output[0]);
}
#[test]
fn case_1() {
    check_answer(vec![1, 1], vec![2, 2]);
}

#[test]
fn case_2() {
    check_answer(vec![1, 2], vec![2, 3]);
}

#[test]
fn case_3() {
    check_answer(vec![2], vec![1, 3]);
}

#[test]
fn case_4() {
    check_answer(vec![1, 2, 5], vec![2, 4]);
}
