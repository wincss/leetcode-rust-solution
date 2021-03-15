use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::generate_matrix(5),
        vec![
            vec![1, 2, 3, 4, 5],
            vec![16, 17, 18, 19, 6],
            vec![15, 24, 25, 20, 7],
            vec![14, 23, 22, 21, 8],
            vec![13, 12, 11, 10, 9]
        ]
    );
}
