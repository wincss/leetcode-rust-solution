use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::dest_city(vec![
            sv!["London", "New York"],
            sv!["New York", "Lima"],
            sv!["Lima", "Sao Paulo"]
        ]),
        s!("Sao Paulo")
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::dest_city(vec![sv!["B", "C"], sv!["D", "B"], sv!["C", "A"]]),
        s!("A")
    );
}

#[test]
fn case_3() {
    assert_eq!(Solution::dest_city(vec![sv!["A", "Z"]]), s!("Z"));
}
