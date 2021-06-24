use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::open_lock(
            to_string_vec(&["0201", "0101", "0102", "1212", "2002"]),
            "0202".to_string()
        ),
        6
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::open_lock(to_string_vec(&["8888"]), "0009".to_string()),
        1
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::open_lock(
            to_string_vec(&["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"]),
            "8888".to_string()
        ),
        -1
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::open_lock(to_string_vec(&["0000"]), "8888".to_string()),
        -1
    );
}
