use crate::*;

#[test]
fn case_1() {
    let expect = vec![
        1, 2, 3, 8, 10, 36, 41, 132, 250, 700, 750, 4010, 4237, 10680, 24679,
    ];
    for i in 0..15 {
        assert_eq!(Solution::count_arrangement(i + 1), expect[i as usize]);
    }
}
