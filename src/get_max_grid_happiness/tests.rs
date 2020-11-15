use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::get_max_grid_happiness(2, 3, 1, 2), 240);
}

#[test]
fn case_2() {
    assert_eq!(Solution::get_max_grid_happiness(3, 1, 2, 1), 260);
}

#[test]
fn case_3() {
    assert_eq!(Solution::get_max_grid_happiness(2, 2, 4, 0), 240);
}

#[test]
fn case_4() {
    assert_eq!(Solution::get_max_grid_happiness(5, 5, 6, 6), 1240);
}
