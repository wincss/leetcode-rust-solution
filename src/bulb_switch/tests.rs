use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::bulb_switch(0), 0);
}

#[test]
fn case_2() {
    assert_eq!(Solution::bulb_switch(1), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::bulb_switch(1000000000), 31622);
}
