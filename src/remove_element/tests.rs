use crate::*;

#[test]
fn case_1() {
    let mut input = vec![3, 2, 2, 3];
    let length = Solution::remove_element(&mut input, 3) as usize;
    let mut output: Vec<i32> = input.drain(0..length).collect();
    output.sort();
    assert_eq!(output, vec![2, 2]);
}

#[test]
fn case_2() {
    let mut input = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let length = Solution::remove_element(&mut input, 2) as usize;
    let mut output: Vec<i32> = input.drain(0..length).collect();
    output.sort();
    assert_eq!(output, vec![0, 0, 1, 3, 4]);
}
