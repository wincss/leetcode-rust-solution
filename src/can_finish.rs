use crate::*;
use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut indeg = vec![0; num_courses];
        let mut pre = vec![];
        for _ in 0..num_courses {
            pre.push(vec![]);
        }
        for pair in prerequisites.iter() {
            pre[pair[1] as usize].push(pair[0] as usize);
            indeg[pair[0] as usize] += 1;
        }
        let mut learn_courses = 0;
        let mut queue = VecDeque::new();
        for i in 0..num_courses {
            if indeg[i] == 0 {
                learn_courses += 1;
                queue.push_back(i);
            }
        }
        while let Some(v) = queue.pop_front() {
            for &i in pre[v].iter() {
                indeg[i] -= 1;
                if indeg[i] == 0 {
                    learn_courses += 1;
                    queue.push_back(i);
                }
            }
        }
        learn_courses == num_courses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    }
}
