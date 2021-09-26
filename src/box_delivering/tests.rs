use crate::*;

use serde_json;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::PathBuf,
};

#[test]
fn case_1() {
    assert_eq!(
        Solution::box_delivering(vv![[1, 1], [2, 1], [1, 1]], 2, 3, 3),
        4
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::box_delivering(vv![[1, 2], [3, 3], [3, 1], [3, 1], [2, 4]], 3, 3, 6),
        6
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::box_delivering(vv![[1, 4], [1, 2], [2, 1], [2, 1], [3, 2], [3, 4]], 3, 6, 7),
        6
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::box_delivering(
            vv![
                [2, 4],
                [2, 5],
                [3, 1],
                [3, 2],
                [3, 7],
                [3, 1],
                [4, 4],
                [1, 3],
                [5, 2]
            ],
            5,
            5,
            7
        ),
        14
    );
}

fn testcase_from_file(filename: &str) -> Result<(), Error> {
    let mut filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filepath.push(filename);
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let boxes: Vec<Vec<i32>> = serde_json::from_str(&lines.next().unwrap()?[..])?;
    let ports_count: i32 = lines.next().unwrap()?.parse().unwrap();
    let max_boxes: i32 = lines.next().unwrap()?.parse().unwrap();
    let max_weight: i32 = lines.next().unwrap()?.parse().unwrap();
    let output: i32 = lines.next().unwrap()?.parse().unwrap();

    assert_eq!(
        Solution::box_delivering(boxes, ports_count, max_boxes, max_weight),
        output,
    );
    Ok(())
}

#[test]
fn case_5() -> Result<(), Error> {
    testcase_from_file("src/box_delivering/case_38.in")
}

#[test]
fn case_6() -> Result<(), Error> {
    testcase_from_file("src/box_delivering/case_40.in")
}
