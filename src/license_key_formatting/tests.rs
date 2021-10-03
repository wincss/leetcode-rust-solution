use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::license_key_formatting(s!("5F3Z-2e-9-w"), 4),
        s!("5F3Z-2E9W")
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::license_key_formatting(s!("2-5g-3-J"), 2),
        s!("2-5G-3J")
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::license_key_formatting(s!("2-4A0r7-4k"), 3),
        s!("24-A0R-74K")
    );
}

#[test]
fn case_4() {
    assert_eq!(Solution::license_key_formatting(s!("---"), 3), s!(""));
}
