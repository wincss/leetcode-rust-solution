use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_cheapest_price(3, vv![[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 1),
        200
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_cheapest_price(3, vv![[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 0),
        500
    );
}
#[test]
fn case_3() {
    assert_eq!(
        Solution::find_cheapest_price(
            5,
            vv![
                [0, 1, 5],
                [1, 2, 5],
                [0, 3, 2],
                [3, 1, 2],
                [1, 4, 1],
                [4, 2, 1]
            ],
            0,
            2,
            2
        ),
        7
    );
}
#[test]
fn case_4() {
    assert_eq!(
        Solution::find_cheapest_price(4, vv![[0, 1, 1], [0, 2, 5], [1, 2, 1], [2, 3, 1]], 0, 3, 1),
        6
    );
}
