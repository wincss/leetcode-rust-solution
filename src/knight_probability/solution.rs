use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn knight_probability(n: i32, k: i32, r: i32, c: i32) -> f64 {
        let offset = [
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
        ];
        let mut probability = HashMap::new();
        probability.insert((r as usize, c as usize), 1.0);
        let mut container = Some(probability);
        for _ in 0..k {
            let old = container.take().unwrap();
            let mut new = HashMap::new();
            for ((x, y), p) in old.iter() {
                let x = *x as i32;
                let y = *y as i32;
                for (dx, dy) in offset.iter() {
                    if x + *dx >= 0 && x + *dx < n && y + *dy >= 0 && y + *dy < n {
                        let nx = (x + *dx) as usize;
                        let ny = (y + *dy) as usize;
                        let f = new.entry((nx, ny)).or_insert(0_f64);
                        *f += *p / 8.0;
                    }
                }
            }
            container.replace(new);
        }
        container.take().unwrap().values().sum()
    }
}
