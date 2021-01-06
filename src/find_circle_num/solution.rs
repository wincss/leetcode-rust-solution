use crate::*;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
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
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let n = is_connected.len();
        for i in 0..n {
            for j in 0..n {
                if is_connected[i][j] == 1 {
                    union(&i, &j, &mut parent, &mut size);
                }
            }
        }
        let mut s = HashSet::new();
        for i in 0..n {
            s.insert(find(&i, &mut parent, &mut size));
        }
        s.len() as i32
    }
}
