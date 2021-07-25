use crate::*;

fn check_output(input: Vec<Vec<i32>>) {
    let output = Solution::restore_array(input.clone());
    let mut input = input;
    for i in input.iter_mut() {
        i.sort();
    }
    input.sort();
    let mut pairs = vec![];
    let n = output.len();
    for i in 1..n {
        pairs.push(vec![
            output[i].min(output[i - 1]),
            output[i].max(output[i - 1]),
        ]);
    }
    pairs.sort();
    assert_eq!(input, pairs);
}

#[test]
fn case_1() {
    check_output(vec![vec![2, 1], vec![3, 4], vec![3, 2]]);
}

#[test]
fn case_2() {
    check_output(vec![vec![4, -2], vec![1, 4], vec![-3, 1]]);
}

#[test]
fn case_3() {
    check_output(vec![vec![100000, -100000]]);
}
