use crate::*;

use std::collections::VecDeque;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut num_courses = num_courses as usize;
        let mut indegree = vec![0; num_courses];
        let mut edge = vec![vec![]; num_courses];
        for i in prerequisites {
            edge[i[1] as usize].push(i[0] as usize);
            indegree[i[0] as usize] += 1;
        }
        let mut queue = VecDeque::new();
        for i in 0..num_courses {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }
        num_courses -= queue.len();
        let mut result = vec![];
        while let Some(v) = queue.pop_front() {
            result.push(v as i32);
            for &i in edge[v].iter() {
                indegree[i] -= 1;
                if indegree[i] == 0 {
                    queue.push_back(i);
                    num_courses -= 1;
                }
            }
        }
        if num_courses == 0 {
            result
        } else {
            vec![]
        }
    }
}
