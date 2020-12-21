use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn distance_limited_paths_exist(
        _: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
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
        fn union<T>(x: &T, y: &T, parent: &mut HashMap<T, T>, size: &mut HashMap<T, usize>)
        where
            T: std::cmp::Eq + std::hash::Hash + Clone,
        {
            let x = find(x, parent, size);
            let y = find(y, parent, size);
            if size[&x] > size[&y] {
                parent.insert(y.clone(), x.clone());
                size.insert(x.clone(), size[&x] + size[&y]);
            } else {
                parent.insert(x.clone(), y.clone());
                size.insert(y.clone(), size[&x] + size[&y]);
            }
        }

        let n = edge_list.len();
        let mut edges = edge_list;
        let mut queries: Vec<_> = queries.into_iter().enumerate().collect();
        edges.sort_by_key(|v| v[2]);
        queries.sort_by_key(|(_, v)| v[2]);

        let mut result = vec![false; queries.len()];

        let mut parent = HashMap::new();
        let mut size = HashMap::new();

        let mut edge_idx = 0_usize;
        for (i, v) in queries {
            while edge_idx < n && edges[edge_idx][2] < v[2] {
                union(
                    &edges[edge_idx][0],
                    &edges[edge_idx][1],
                    &mut parent,
                    &mut size,
                );
                edge_idx += 1;
            }
            result[i] = find(&v[0], &mut parent, &mut size) == find(&v[1], &mut parent, &mut size);
        }

        result
    }
}
