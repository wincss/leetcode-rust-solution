use crate::*;

impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        const MOD: i64 = 1000000007;
        let n = next_visit.len();
        let mut first = vec![];
        let mut day: i64 = 0;
        for i in 0..n - 1 {
            first.push(day);
            // `1` for next_visit[i]
            // `day - first[next_visit[i]]` for back to i
            // `1` for i + 1
            day = (day + 2 + day - first[next_visit[i] as usize]).rem_euclid(MOD);
        }
        day as i32
    }
}
