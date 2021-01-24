use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::regions_by_slashes(to_string_vec(&[" /", "/ "])),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::regions_by_slashes(to_string_vec(&[" /", "  "])),
        1
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::regions_by_slashes(to_string_vec(&["\\/", "/\\"])),
        4
    );
}
#[test]
fn case_4() {
    assert_eq!(
        Solution::regions_by_slashes(to_string_vec(&["/\\", "\\/"])),
        5
    );
}
#[test]
fn case_5() {
    assert_eq!(
        Solution::regions_by_slashes(to_string_vec(&["//", "/ "])),
        3
    );
}
