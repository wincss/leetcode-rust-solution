use super::solution::*;
use crate::*;

#[test]
fn case_1() {
    let mut ranges = SummaryRanges::new();
    ranges.add_num(1);
    assert_eq!(ranges.get_intervals(), vv![[1, 1]]);
    ranges.add_num(3);
    assert_eq!(ranges.get_intervals(), vv![[1, 1], [3, 3]]);
    ranges.add_num(7);
    assert_eq!(ranges.get_intervals(), vv![[1, 1], [3, 3], [7, 7]]);
    ranges.add_num(2);
    assert_eq!(ranges.get_intervals(), vv![[1, 3], [7, 7]]);
    ranges.add_num(6);
    assert_eq!(ranges.get_intervals(), vv![[1, 3], [6, 7]]);
}
