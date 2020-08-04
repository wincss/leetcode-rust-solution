use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::minimal_steps(to_string_vec(&["S#O", "M..", "M.T"])),
        16
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::minimal_steps(to_string_vec(&["S#O", "M.#", "M.T"])),
        -1
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::minimal_steps(to_string_vec(&["S#O", "M.T", "M.."])),
        17
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::minimal_steps(to_string_vec(&["MMMMM", "MS#MM", "MM#TO"])),
        95
    );
}
