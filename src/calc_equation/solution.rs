use crate::*;

use std::collections::HashMap;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        fn find<'a, T>(
            x: &'a T,
            scale: &mut HashMap<&'a T, f64>,
            parent: &mut HashMap<&'a T, &'a T>,
            size: &mut HashMap<&'a T, usize>,
        ) -> (&'a T, f64)
        where
            T: std::cmp::Eq + std::hash::Hash,
        {
            let f = parent.entry(x).or_insert(x);
            size.entry(x).or_insert(1);
            scale.entry(x).or_insert(1.0_f64);
            if **f == *x {
                (x, 1.0_f64)
            } else {
                let (p, s) = find(*f, scale, parent, size);
                parent.insert(x, p);
                let s = s * scale[&x];
                scale.insert(x, s);
                (p, s)
            }
        }
        fn union<'a, T>(
            x: &'a T,
            y: &'a T,
            s: f64,
            scale: &mut HashMap<&'a T, f64>,
            parent: &mut HashMap<&'a T, &'a T>,
            size: &mut HashMap<&'a T, usize>,
        ) where
            T: std::cmp::Eq + std::hash::Hash,
        {
            let (x, s1) = find(x, scale, parent, size);
            let (y, s2) = find(y, scale, parent, size);
            if size[&x] > size[&y] {
                parent.insert(y, x);
                size.insert(x, size[&x] + size[&y]);
                // x * s1 = new_x
                // y * s2 = new_y
                // x * s = y
                // new_x / s1 * s = new_y / s2
                // new_x = new_y * s1 / s2 / s
                scale.insert(y, s1 / s2 / s);
            } else {
                parent.insert(x, y);
                size.insert(y, size[&x] + size[&y]);
                // new_y = new_x * s * s2 / s1
                scale.insert(x, s * s2 / s1);
            }
        }
        let mut parent = HashMap::new();
        let mut scale = HashMap::new();
        let mut size = HashMap::new();
        let n = equations.len();
        for i in 0..n {
            union(
                &equations[i][1],
                &equations[i][0],
                values[i],
                &mut scale,
                &mut parent,
                &mut size,
            );
        }
        let mut result = vec![];
        for item in queries.iter() {
            if !(parent.contains_key(&item[0]) && parent.contains_key(&item[1])) {
                result.push(-1.0_f64);
            } else {
                let (p1, s1) = find(&item[0], &mut scale, &mut parent, &mut size);
                let (p2, s2) = find(&item[1], &mut scale, &mut parent, &mut size);
                if p1 == p2 {
                    result.push(s2 / s1);
                } else {
                    result.push(-1.0_f64);
                }
            }
        }
        result
    }
}
