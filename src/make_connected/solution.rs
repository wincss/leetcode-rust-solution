use crate::*;

use std::collections::HashMap;

use common::algorithms::union_find::*;

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let mut free = 0;
        let mut area = n;
        for edge in connections {
            let v1 = find(&edge[0], &mut parent, &mut size);
            let v2 = find(&edge[1], &mut parent, &mut size);
            if v1 == v2 {
                free += 1;
            } else {
                union(&v1, &v2, &mut parent, &mut size);
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
