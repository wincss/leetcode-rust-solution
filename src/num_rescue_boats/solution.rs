use crate::*;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort();
        let n = people.len();
        let mut left = 0;
        let mut right = n;
        let mut result = 0;
        while left < right {
            if people[left] + people[right - 1] <= limit {
                left += 1;
                right -= 1;
                result += 1;
            } else {
                right -= 1;
                result += 1;
            }
        }
        result
    }
}
