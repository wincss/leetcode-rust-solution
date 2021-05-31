use crate::*;

impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut last = 0;
        let mut s = vec![0];
        for &i in candies_count.iter() {
            last += i as i64;
            s.push(last);
        }
        let mut result = vec![];
        for item in queries {
            let favority_type = item[0] as usize;
            let favority_day = item[1] as i64;
            let limit = item[2] as i64;
            result.push(
                favority_day < s[favority_type + 1]
                    && limit * (favority_day + 1) > s[favority_type],
            )
        }
        result
    }
}
