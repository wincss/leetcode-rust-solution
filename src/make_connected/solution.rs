use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        fn find(x: i32, parent: &mut HashMap<i32, i32>, size: &mut HashMap<i32, i32>) -> i32 {
            let f = parent.entry(x).or_insert(x);
            size.entry(x).or_insert(1);
            if *f == x {
                x
            } else {
                let p = find(parent[&x], parent, size);
                parent.insert(x, p);
                parent[&x]
            }
        }
        fn union(x: i32, y: i32, parent: &mut HashMap<i32, i32>, size: &mut HashMap<i32, i32>) {
            let x = find(x, parent, size);
            let y = find(y, parent, size);
            if size[&x] > size[&y] {
                parent.insert(y, x);
                size.insert(x, size[&x] + size[&y]);
            } else {
                parent.insert(x, y);
                size.insert(y, size[&x] + size[&y]);
            }
        }
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let mut free = 0;
        let mut area = n;
        for edge in connections {
            let v1 = find(edge[0], &mut parent, &mut size);
            let v2 = find(edge[1], &mut parent, &mut size);
            if v1 == v2 {
                free += 1;
            } else {
                union(v1, v2, &mut parent, &mut size);
                area -= 1;
            }
        }
        if area - 1 <= free {
            area - 1
        } else {
            -1
        }
    }
}
