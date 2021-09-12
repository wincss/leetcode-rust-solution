use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::traffic_command(sv!["W", "N", "ES", "W"]), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::traffic_command(sv!["NS", "WE", "SE", "EW"]), 3);
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::traffic_command(sv!["SSSSS", "WWW", "NNNNNNNNN", "EEEEEEEEEE"]),
        19
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::traffic_command(sv![
            "SSSSSSSSSSSSSSSSSSSS",
            "WWWWWWWWWWWWWWWWWWWW",
            "SSSSSSSSSSSSSSSSSSSS",
            "SSSSSSSSSSSSSSSSSSSS"
        ]),
        60
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::traffic_command(sv!["WNWSNSWSNN", "NWENWENWN", "ESNESNENE", "WEEEWWESSS"]),
        18
    );
}
