use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        fn find<T>(x: &T, parent: &mut HashMap<T, T>, size: &mut HashMap<T, usize>) -> T
        where
            T: std::cmp::Eq + std::hash::Hash + Clone,
        {
            let f = parent.entry(x.clone()).or_insert(x.clone());
            size.entry(x.clone()).or_insert(1);
            if *f == *x {
                x.clone()
            } else {
                let p = find(&f.clone(), parent, size);
                parent.insert(x.clone(), p.clone());
                p
            }
        }
        fn union<T>(x: &T, y: &T, parent: &mut HashMap<T, T>, size: &mut HashMap<T, usize>) -> bool
        where
            T: std::cmp::Eq + std::hash::Hash + Clone,
        {
            let x = find(x, parent, size);
            let y = find(y, parent, size);
            if x == y {
                return false;
            }
            if size[&x] > size[&y] {
                parent.insert(y.clone(), x.clone());
                size.insert(x.clone(), size[&x] + size[&y]);
            } else {
                parent.insert(x.clone(), y.clone());
                size.insert(y.clone(), size[&x] + size[&y]);
            }
            true
        }

        let n = points.len();
        let mut edges = vec![];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let distance =
                    (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                edges.push((distance, i, j));
            }
        }
        edges.sort();
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let mut result = 0;
        for (d, x, y) in edges {
            if union(&x, &y, &mut parent, &mut size) {
                result += d;
            }
        }
        result
    }
}
