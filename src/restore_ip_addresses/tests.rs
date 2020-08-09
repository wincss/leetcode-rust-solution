use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::restore_ip_addresses("25525511135".to_string()),
        to_string_vec(&["255.255.11.135", "255.255.111.35"])
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::restore_ip_addresses("010010".to_string()),
        to_string_vec(&["0.10.0.10", "0.100.1.0"])
    );
}
