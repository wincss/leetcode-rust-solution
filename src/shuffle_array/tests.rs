use crate::shuffle_array::solution::Solution;

#[test]
fn case_1() {
    let input = vec![1, 2, 3];
    let mut sorted = input.clone();
    sorted.sort();
    let solution = Solution::new(input.clone());
    let mut shuffle_result = solution.shuffle();
    shuffle_result.sort();
    assert_eq!(shuffle_result, sorted);
    let original_array = solution.reset();
    assert_eq!(original_array, input);
    let mut shuffle_result = solution.shuffle();
    shuffle_result.sort();
    assert_eq!(shuffle_result, sorted);
}
