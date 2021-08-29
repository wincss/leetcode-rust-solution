use super::solution::*;

#[test]
fn case_1() {
    let input = vec![1, 2];
    let n = input.len();
    let picker = RandomPicker::new(vec![1, 2]);
    for _ in 0..100 {
        let idx = picker.pick_index();
        assert!(idx >= 0 && idx < n as i32);
    }
}
