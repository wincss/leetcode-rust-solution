use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::generate(0), Vec::<Vec<i32>>::new());
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::generate(5),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ]
    );
}
