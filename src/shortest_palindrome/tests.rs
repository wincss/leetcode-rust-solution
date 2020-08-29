use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::shortest_palindrome(String::from("aacecaaa")),
        String::from("aaacecaaa")
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::shortest_palindrome(String::from("abcd")),
        String::from("dcbabcd")
    );
}

#[test]
fn case_3() {
    let input = String::from_utf8(vec![b'a'; 40002]).unwrap();
    assert_eq!(Solution::shortest_palindrome(input.clone()), input);
}

#[test]
fn case_4() {
    assert_eq!(Solution::shortest_palindrome(String::new()), String::new());
}
