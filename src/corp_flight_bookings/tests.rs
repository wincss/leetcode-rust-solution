use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::corp_flight_bookings(vv![[1, 2, 10], [2, 3, 20], [2, 5, 25]], 5),
        vv![10, 55, 45, 25, 25]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::corp_flight_bookings(vv![[1, 2, 10], [2, 2, 15]], 2),
        vv![10, 25]
    );
}
