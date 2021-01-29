use crate::*;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        // Self::minimum_effort_path_by_union_find(heights)
        Self::minimum_effort_path_by_dijkstra(heights)
        // Self::minimum_effort_path_by_bisect(heights)
    }

    pub fn minimum_effort_path_by_bisect(heights: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashSet, VecDeque};

        let n = heights.len();
        if n == 0 {
            return 0;
        }
        let m = heights[0].len();
        if m == 0 || (n == 1 && m == 1) {
            return 0;
        }
        let mut l = std::i32::MAX;
        let mut r = 0;
        for i in 0..n {
            for j in 0..m {
                if i > 0 {
                    let c = (heights[i][j] - heights[i - 1][j]).abs();
                    l = std::cmp::min(c, l);
                    r = std::cmp::max(c, r);
                }
                if j > 0 {
                    let c = (heights[i][j] - heights[i][j - 1]).abs();
                    l = std::cmp::min(c, l);
                    r = std::cmp::max(c, r);
                }
            }
        }
        fn can_reach_dest(limit: i32, heights: &Vec<Vec<i32>>, n: usize, m: usize) -> bool {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            visited.insert(0);
            queue.push_back(0);
            while let Some(node) = queue.pop_front() {
                if node == n * m - 1 {
                    return true;
                }
                let x = node / m;
                let y = node % m;
                for &(dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if (heights[x][y] - heights[nx][ny]).abs() > limit {
                        continue;
                    }
                    let next_node = nx * m + ny;
                    if visited.contains(&next_node) {
                        continue;
                    }
                    visited.insert(next_node);
                    queue.push_back(next_node);
                }
            }
            false
        }
        while l < r {
            let mid = (l + r) / 2;
            if can_reach_dest(mid, &heights, n, m) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }

    pub fn minimum_effort_path_by_dijkstra(heights: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = heights.len();
        if n == 0 {
            return 0;
        }
        let m = heights[0].len();
        if m == 0 || (n == 1 && m == 1) {
            return 0;
        }
        let nodes = m * n;
        let mut short_path = vec![std::i32::MAX; nodes];
        let mut queue = BinaryHeap::new();
        short_path[0] = 0;
        queue.push(Reverse((0, 0)));
        while let Some(Reverse((dist, node))) = queue.pop() {
            if short_path[node] < dist {
                continue;
            }
            let x = node / m;
            let y = node % m;
            for &(dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                    continue;
                }
                let next_node = (nx * m as i32 + ny) as usize;
                let cost = (heights[x][y] - heights[nx as usize][ny as usize])
                    .abs()
                    .max(dist);

                if short_path[next_node] > cost {
                    short_path[next_node] = cost;
                    queue.push(Reverse((cost, next_node)));
                }
            }
        }
        short_path[nodes - 1]
    }

    pub fn minimum_effort_path_by_union_find(heights: Vec<Vec<i32>>) -> i32 {
        use common::algorithms::union_find::*;
        use std::collections::HashMap;

        let n = heights.len();
        if n == 0 {
            return 0;
        }
        let m = heights[0].len();
        if m == 0 || (n == 1 && m == 1) {
            return 0;
        }
        let mut edges = vec![];
        for i in 0..n {
            for j in 0..m {
                if i > 0 {
                    edges.push((
                        (heights[i][j] - heights[i - 1][j]).abs(),
                        (i - 1) * m + j,
                        i * m + j,
                    ));
                }
                if j > 0 {
                    edges.push((
                        (heights[i][j] - heights[i][j - 1]).abs(),
                        i * m + j - 1,
                        i * m + j,
                    ));
                }
            }
        }
        edges.sort();
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let dest = m * n - 1;
        for &(c, x, y) in edges.iter() {
            union(&x, &y, &mut parent, &mut size);
            if find(&0, &mut parent, &mut size) == find(&dest, &mut parent, &mut size) {
                return c;
            }
        }
        unreachable!();
    }
}
