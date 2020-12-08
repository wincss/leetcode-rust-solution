use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::split_into_fibonacci(String::from("123456579")),
        vec![123, 456, 579]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::split_into_fibonacci(String::from("11235813")),
        vec![1, 1, 2, 3, 5, 8, 13]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::split_into_fibonacci(String::from("112358130")),
        Vec::<i32>::new()
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::split_into_fibonacci(String::from("0123")),
        Vec::<i32>::new()
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::split_into_fibonacci(String::from("1012")),
        Vec::<i32>::new()
    );
}

#[test]
fn case_6() {
    assert_eq!(
        Solution::split_into_fibonacci(String::from("5511816597")),
        Vec::<i32>::new()
    );
}

// TODO: write test case for 1101111 [110, 1, 111] [11,0,11,11]
