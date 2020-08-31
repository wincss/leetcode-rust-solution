use crate::*;

use std::collections::VecDeque;
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut locked = rooms.len();
        let mut unlocked = vec![false; locked];
        let mut queue = VecDeque::new();
        locked -= 1;
        unlocked[0] = true;
        queue.push_back(0);
        while let Some(v) = queue.pop_front() {
            for &i in rooms[v].iter() {
                if !unlocked[i as usize] {
                    locked -= 1;
                    unlocked[i as usize] = true;
                    queue.push_back(i as usize)
                }
            }
        }
        locked == 0
    }
}
