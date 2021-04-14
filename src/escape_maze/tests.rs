use crate::*;

#[test]
fn case_1() {
    assert!(Solution::escape_maze(vec![
        to_string_vec(&[".#.", "#.."]),
        to_string_vec(&["...", ".#."]),
        to_string_vec(&[".##", ".#."]),
        to_string_vec(&["..#", ".#."])
    ]));
}

#[test]
fn case_2() {
    assert!(!Solution::escape_maze(vec![
        to_string_vec(&[".#.", "..."]),
        to_string_vec(&["...", "..."]),
    ]));
}

#[test]
fn case_3() {
    assert!(!Solution::escape_maze(vec![
        to_string_vec(&["...", "...", "..."]),
        to_string_vec(&[".##", "###", "##."]),
        to_string_vec(&[".##", "###", "##."]),
        to_string_vec(&[".##", "###", "##."]),
        to_string_vec(&[".##", "###", "##."]),
        to_string_vec(&[".##", "###", "##."]),
        to_string_vec(&[".##", "###", "##."])
    ]));
}

#[test]
fn case_4() {
    assert!(Solution::escape_maze(vec![
        to_string_vec(&["...", ".##", "##.", "#.#", ".##", "...", ".#.", "##.", "##."]),
        to_string_vec(&[".##", "###", "##.", ".##", ".##", "##.", ".#.", "###", "##."]),
        to_string_vec(&[".##", "###", "###", ".##", "##.", "##.", "#.#", "###", ".#."]),
        to_string_vec(&[".##", "#.#", ".##", "#.#", "###", "#.#", "###", "...", ".#."]),
        to_string_vec(&["..#", "###", "#..", ".##", ".##", "..#", ".#.", "###", ".#."]),
        to_string_vec(&["..#", ".#.", "..#", ".##", "###", "#.#", "#..", "###", "##."]),
        to_string_vec(&[".##", "..#", ".##", ".#.", "##.", "###", "##.", "###", "##."]),
        to_string_vec(&[".##", "..#", "#.#", ".##", "###", ".##", "##.", ".##", "#.."]),
        to_string_vec(&["...", "##.", "#.#", "..#", "##.", "..#", ".##", "#.#", "##."]),
        to_string_vec(&[".##", "###", "###", "#.#", ".##", ".##", "###", "###", "#.."]),
        to_string_vec(&["..#", "###", "..#", "#..", ".#.", "###", "#.#", "###", ".#."]),
        to_string_vec(&["..#", "###", "##.", "##.", ".#.", "#..", "###", "##.", ".#."]),
        to_string_vec(&[".##", "#.#", ".#.", ".##", ".#.", "###", ".#.", "###", "##."]),
        to_string_vec(&[".#.", "#.#", ".##", "#..", "#.#", "##.", "###", "###", "##."]),
        to_string_vec(&["..#", "###", "###", "###", "##.", ".#.", "##.", "###", "##."]),
        to_string_vec(&["..#", "#..", ".#.", "##.", "###", "...", ".##", "#.#", ".#."]),
        to_string_vec(&[".##", "###", ".##", ".##", "###", "#..", "###", "...", ".#."]),
        to_string_vec(&[".##", ".##", "##.", "#..", "###", "..#", "...", "###", "##."]),
        to_string_vec(&[".##", "#..", "###", "###", "##.", "#..", ".##", "..#", "##."]),
        to_string_vec(&["...", ".#.", "###", "###", "###", "###", "##.", "#.#", ".#."]),
        to_string_vec(&[".##", ".#.", "#..", "#.#", "###", "##.", ".#.", "###", ".#."]),
        to_string_vec(&["..#", "###", "###", "###", "#.#", "##.", "##.", "#.#", "#.."]),
        to_string_vec(&["...", "###", "###", "##.", "#.#", ".#.", "#.#", "#..", ".#."]),
        to_string_vec(&[".##", "..#", "##.", ".##", "###", "#..", "#.#", "#.#", "##."]),
        to_string_vec(&["..#", "###", "###", "##.", ".##", ".##", "#.#", "#..", ".#."]),
        to_string_vec(&["..#", "..#", "#..", "...", ".##", "##.", "#.#", "##.", "##."]),
        to_string_vec(&[".#.", "##.", "#.#", "##.", "#.#", "##.", "###", ".#.", "##."]),
        to_string_vec(&[".#.", "###", "..#", "###", ".##", "..#", ".#.", "###", "##."]),
        to_string_vec(&[".#.", "###", "###", "##.", "###", ".##", "##.", "##.", ".#."]),
        to_string_vec(&[".#.", ".##", "###", ".#.", ".#.", ".#.", "###", "###", "..."]),
        to_string_vec(&[".##", "#.#", ".##", "###", "###", ".##", ".##", ".##", "##."]),
        to_string_vec(&["..#", "##.", ".#.", ".##", "###", "###", "###", ".##", "##."]),
        to_string_vec(&[".#.", ".##", "##.", "#..", "###", ".#.", ".#.", "#.#", "##."]),
        to_string_vec(&[".#.", "###", "...", "###", "##.", ".#.", "#.#", ".##", "##."]),
        to_string_vec(&["..#", "#..", "#.#", ".##", "###", "##.", ".#.", "##.", "##."]),
        to_string_vec(&["..#", "##.", "###", "###", "#..", "##.", "##.", ".##", "##."]),
        to_string_vec(&["..#", "###", ".##", "..#", ".#.", ".##", ".##", "###", ".#."]),
        to_string_vec(&["..#", ".#.", ".##", ".#.", ".#.", ".##", "#.#", "##.", "##."]),
        to_string_vec(&["..#", "###", "#..", "...", "#..", ".##", "..#", ".##", "..."]),
        to_string_vec(&[".##", "###", "#..", "###", "#.#", "##.", "#..", "###", "##."])
    ]));
}

#[test]
fn case_5() {
    assert!(Solution::escape_maze(vec![
        to_string_vec(&[".##..####", ".#######."]),
        to_string_vec(&["..######.", "########."]),
        to_string_vec(&[".#####.##", ".#######."]),
        to_string_vec(&[".#..###.#", "########."]),
        to_string_vec(&[".########", "########."]),
        to_string_vec(&[".######.#", "####.###."]),
        to_string_vec(&[".#####.##", "#####.#.."]),
        to_string_vec(&[".##.####.", "##.#####."]),
        to_string_vec(&[".########", "#####.##."]),
        to_string_vec(&[".#.######", "#.##.###."]),
        to_string_vec(&[".########", "###.#.#.."]),
        to_string_vec(&[".########", "########."]),
        to_string_vec(&[".####.##.", "##.##...."]),
        to_string_vec(&[".#######.", "###.#.##."]),
        to_string_vec(&[".####.###", "###.####."]),
        to_string_vec(&[".######.#", "##.####.."]),
        to_string_vec(&[".##.#####", "##.###.#."]),
        to_string_vec(&[".####.###", "##.#####."]),
        to_string_vec(&[".##.##..#", ".#.#####."]),
        to_string_vec(&[".###.####", "##.#..##."]),
        to_string_vec(&[".####.#.#", "##.#####."]),
        to_string_vec(&[".####.###", "####.###."]),
        to_string_vec(&[".########", "#######.."]),
        to_string_vec(&[".#####.##", "#.######."]),
        to_string_vec(&[".########", "###..#.#."]),
        to_string_vec(&[".####.#.#", "###..##.."]),
        to_string_vec(&[".######.#", "########."]),
        to_string_vec(&[".########", "##.#####."]),
        to_string_vec(&[".########", "..######."]),
        to_string_vec(&[".#####..#", "#######.."]),
        to_string_vec(&[".#.######", ".#######."]),
        to_string_vec(&[".###.#.#.", ".##..#.#."]),
        to_string_vec(&[".#.##.###", "####.##.."])
    ]));
}

#[test]
fn case_6() {
    assert!(Solution::escape_maze(vec![
        to_string_vec(&["....###.", "###.#.##", ".##..##."]),
        to_string_vec(&[".#####..", "##.####.", "##.####."]),
        to_string_vec(&["....####", "###..###", "##..##.."]),
        to_string_vec(&[".####...", "######.#", "###.##.."]),
        to_string_vec(&["..###.##", "########", "#######."]),
        to_string_vec(&["...##.##", "###.####", ".#.#.#.."]),
        to_string_vec(&[".######.", "#.#.....", "#.#.#.#."]),
        to_string_vec(&[".###.##.", "##.#####", "###.##.."]),
        to_string_vec(&["..#.####", "#####.##", "##.###.."]),
        to_string_vec(&[".#.###.#", ".#######", "#####.#."]),
        to_string_vec(&[".######.", "####....", ".##..##."]),
        to_string_vec(&[".###.#..", "###.#.#.", "#####.#."]),
        to_string_vec(&[".###.###", "###.####", "....###."]),
        to_string_vec(&[".###.##.", "########", "#####.#."]),
        to_string_vec(&[".###.###", "##.####.", ".###...."]),
        to_string_vec(&[".#.#.##.", ".##.####", "#####.#."]),
        to_string_vec(&["..#.####", "#.##....", "####...."]),
        to_string_vec(&["..#.##.#", "#.##..#.", "###.###."]),
        to_string_vec(&["..##.#.#", ".##.#..#", ".####..."]),
        to_string_vec(&[".##..##.", "########", "#####.#."]),
        to_string_vec(&[".####.##", "#.#...##", "#.##..#."]),
        to_string_vec(&["..#.####", "######.#", "###.###."]),
        to_string_vec(&[".#..#..#", "###..##.", "#..#...."])
    ]));
}

#[test]
fn case_7() {
    assert!(Solution::escape_maze(vec![
        vec!["..................................................".to_string(); 50]; 100
    ]));
}