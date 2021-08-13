use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut pref = vec![vec![0; n]; n];
        for i in 0..n {
            for (j, &v) in preferences[i].iter().enumerate() {
                pref[i][v as usize] = j;
            }
        }
        let mut unhappy = HashSet::new();
        let n = pairs.len();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let test = |x: i32, y: i32, u: i32, v: i32| {
                    pref[x as usize][u as usize] < pref[x as usize][y as usize]
                        && pref[u as usize][x as usize] < pref[u as usize][v as usize]
                };
                if test(pairs[i][0], pairs[i][1], pairs[j][0], pairs[j][1]) {
                    unhappy.insert(pairs[i][0]);
                }
                if test(pairs[i][0], pairs[i][1], pairs[j][1], pairs[j][0]) {
                    unhappy.insert(pairs[i][0]);
                }
                if test(pairs[i][1], pairs[i][0], pairs[j][0], pairs[j][1]) {
                    unhappy.insert(pairs[i][1]);
                }
                if test(pairs[i][1], pairs[i][0], pairs[j][1], pairs[j][0]) {
                    unhappy.insert(pairs[i][1]);
                }
            }
        }
        unhappy.len() as i32
    }
}
