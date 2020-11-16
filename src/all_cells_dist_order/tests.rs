use crate::*;

fn check_output(r: i32, c: i32, r0: i32, c0: i32) {
    let output = Solution::all_cells_dist_order(r, c, r0, c0);
    assert_eq!(output.len() as i32, r * c);
    let mut last_dist = 0;
    for i in output.iter() {
        let dist = (i[0] - r0).abs() + (i[1] - c0).abs();
        assert!(dist >= last_dist);
        last_dist = dist;
    }
    // checking every elements is trivial, to be done later
}
#[test]
fn case_1() {
    check_output(1, 2, 0, 0);
}

#[test]
fn case_2() {
    check_output(2, 2, 0, 1);
}
