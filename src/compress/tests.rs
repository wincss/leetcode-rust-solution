use crate::*;

#[test]
fn case_1() {
    let mut chars: Vec<char> = s!("aabbccc").chars().collect();
    assert_eq!(Solution::compress(&mut chars), 6);
    assert_eq!(chars[..6], ['a', '2', 'b', '2', 'c', '3']);
}

#[test]
fn case_2() {
    let mut chars: Vec<char> = s!("a").chars().collect();
    assert_eq!(Solution::compress(&mut chars), 1);
    assert_eq!(chars[..1], ['a']);
}

#[test]
fn case_3() {
    let mut chars: Vec<char> = s!("abbbbbbbbbbbb").chars().collect();
    assert_eq!(Solution::compress(&mut chars), 4);
    assert_eq!(chars[..4], ['a', 'b', '1', '2']);
}

#[test]
fn case_4() {
    let mut chars: Vec<char> = s!("aaabbaa").chars().collect();
    assert_eq!(Solution::compress(&mut chars), 6);
    assert_eq!(chars[..6], ['a', '3', 'b', '2', 'a', '2']);
}
