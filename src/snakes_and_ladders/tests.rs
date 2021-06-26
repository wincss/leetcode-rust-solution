use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::snakes_and_ladders(vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1]
        ]),
        4
    );
}
