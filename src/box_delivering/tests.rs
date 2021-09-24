use crate::*;

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

#[test]
fn case_5() -> Result<(), Error> {
    let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filename.push("src/box_delivering/case_38.in");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }

    Ok(())
}
