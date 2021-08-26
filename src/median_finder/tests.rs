use super::solution::*;

#[test]
fn case_1() {
    let mut m = MedianFinder::new();
    m.add_num(1);
    m.add_num(2);
    assert_eq!(m.find_median(), 1.5_f64);
    m.add_num(3);
    assert_eq!(m.find_median(), 2_f64);
}
