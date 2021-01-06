use crate::*;

use std::collections::HashMap;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        fn find(
            x: String,
            scale: &mut HashMap<String, f64>,
            parent: &mut HashMap<String, String>,
            size: &mut HashMap<String, usize>,
        ) -> (String, f64) {
            let f = parent.entry(x.clone()).or_insert(x.clone());
            size.entry(x.clone()).or_insert(1);
            scale.entry(x.clone()).or_insert(1.0_f64);
            if *f == x {
                (x, 1.0_f64)
            } else {
                let (p, s) = find(parent[&x].clone(), scale, parent, size);
                parent.insert(x.clone(), p);
                let s = s * scale[&x];
                scale.insert(x.clone(), s);
                (parent[&x].clone(), s)
            }
        }
        fn union(
            x: String,
            y: String,
            s: f64,
            scale: &mut HashMap<String, f64>,
            parent: &mut HashMap<String, String>,
            size: &mut HashMap<String, usize>,
        ) {
            let (x, s1) = find(x, scale, parent, size);
            let (y, s2) = find(y, scale, parent, size);
            if size[&x] > size[&y] {
                parent.insert(y.clone(), x.clone());
                size.insert(x.clone(), size[&x] + size[&y]);
                // x * s1 = new_x
                // y * s2 = new_y
                // x * s = y
                // new_x / s1 * s = new_y / s2
                // new_x = new_y * s1 / s2 / s
                scale.insert(y.clone(), s1 / s2 / s);
            } else {
                parent.insert(x.clone(), y.clone());
                size.insert(y.clone(), size[&x] + size[&y]);
                // new_y = new_x * s * s2 / s1
                scale.insert(x.clone(), s * s2 / s1);
            }
        }
        let mut parent = HashMap::new();
        let mut scale = HashMap::new();
        let mut size = HashMap::new();
        let n = equations.len();
        for i in 0..n {
            union(
                equations[i][1].clone(),
                equations[i][0].clone(),
                values[i],
                &mut scale,
                &mut parent,
                &mut size,
            );
        }
        let mut result = vec![];
        for item in queries {
            if !(parent.contains_key(&item[0]) && parent.contains_key(&item[1])) {
                result.push(-1.0_f64);
            } else {
                let (p1, s1) = find(item[0].clone(), &mut scale, &mut parent, &mut size);
                let (p2, s2) = find(item[1].clone(), &mut scale, &mut parent, &mut size);
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
