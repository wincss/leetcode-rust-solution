use crate::*;

#[test]
fn case_1() {
    let mut input = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut input);
    assert_eq!(input, vec!['o', 'l', 'l', 'e', 'h']);
}

#[test]
fn case_2() {
    let mut input = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    Solution::reverse_string(&mut input);
    assert_eq!(input, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}
