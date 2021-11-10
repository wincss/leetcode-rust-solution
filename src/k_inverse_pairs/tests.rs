use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        (0..2)
            .map(|k| Solution::k_inverse_pairs(1, k))
            .collect::<Vec<i32>>(),
        vec![1, 0]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        (0..3)
            .map(|k| Solution::k_inverse_pairs(2, k))
            .collect::<Vec<i32>>(),
        vec![1, 1, 0]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        (0..5)
            .map(|k| Solution::k_inverse_pairs(3, k))
            .collect::<Vec<i32>>(),
        vec![1, 2, 2, 1, 0]
    );
}

#[test]
fn case_4() {
    assert_eq!(
        (0..8)
            .map(|k| Solution::k_inverse_pairs(4, k))
            .collect::<Vec<i32>>(),
        vec![1, 3, 5, 6, 5, 3, 1, 0]
    );
}

#[test]
fn case_5() {
    assert_eq!(
        (0..12)
            .map(|k| Solution::k_inverse_pairs(5, k))
            .collect::<Vec<i32>>(),
        vec![1, 4, 9, 15, 20, 22, 20, 15, 9, 4, 1, 0]
    );
}

#[test]
fn case_6() {
    assert_eq!(Solution::k_inverse_pairs(1000, 1000), 663677020);
}
