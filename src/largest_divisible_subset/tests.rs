use crate::*;

/*
checking every elements is trivial, to be done later
*/

#[test]
fn case_1() {
    let output = Solution::largest_divisible_subset(vec![1, 2, 3]);
    println!("{:?}", output);
    assert_eq!(output.len(), 2);
}

#[test]
fn case_2() {
    let output = Solution::largest_divisible_subset(vec![1, 2, 4, 8]);
    println!("{:?}", output);
    assert_eq!(output.len(), 4);
}

#[test]
fn case_3() {
    let output = Solution::largest_divisible_subset(vec![3, 4, 16, 8]);
    println!("{:?}", output);
    assert_eq!(output.len(), 3);
}

#[test]
fn case_4() {
    let output = Solution::largest_divisible_subset(vec![4, 8, 10, 240]);
    println!("{:?}", output);
    assert_eq!(output.len(), 3);
}

#[test]
fn case_5() {
    let output =
        Solution::largest_divisible_subset(vec![5, 9, 18, 54, 108, 540, 90, 180, 360, 720]);
    println!("{:?}", output);
    assert_eq!(output.len(), 6);
}
