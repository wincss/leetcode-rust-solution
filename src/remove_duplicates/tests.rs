use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::remove_duplicates_1047(String::from("abbaca")),
        String::from("ca")
    );
}

#[test]
fn case_2() {
    let mut input = vec![1, 1, 1, 2, 2, 3];
    let length = Solution::remove_duplicates_80(&mut input);
    assert_eq!(&input[0..length as usize], &[1, 1, 2, 2, 3]);
}

#[test]
fn case_3() {
    let mut input = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    let length = Solution::remove_duplicates_80(&mut input);
    assert_eq!(&input[0..length as usize], &[0, 0, 1, 1, 2, 3, 3]);
}

#[test]
fn case_4() {
    let mut input = vec![1, 1, 2];
    let length = Solution::remove_duplicates_26(&mut input);
    assert_eq!(&input[0..length as usize], &[1, 2]);
}

#[test]
fn case_5() {
    let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let length = Solution::remove_duplicates_26(&mut input);
    assert_eq!(&input[0..length as usize], &[0, 1, 2, 3, 4]);
}
