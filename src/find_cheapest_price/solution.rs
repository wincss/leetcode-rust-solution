use crate::*;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;
        let k = k as usize;

        let mut map = vec![HashMap::new(); n];
        for item in flights.into_iter() {
            map[item[0] as usize].insert(item[1] as usize, item[2]);
        }

        let mut queue = BinaryHeap::new();
        let mut dist = vec![vec![std::i32::MAX; 2 + k as usize]; n];
        dist[src][0] = 0;
        queue.push((Reverse(0), 0, src as usize));
        while let Some((Reverse(d), leg, current)) = queue.pop() {
            if d > dist[current][leg] {
                continue;
            }
            for (&next, &distance) in map[current].iter() {
                if leg <= k && d + distance < dist[next][leg + 1] {
                    dist[next][leg + 1] = d + distance;
                    queue.push((Reverse(d + distance), leg + 1, next));
                }
            }
        }
        let result = *dist[dst].iter().min().unwrap();
        if result < std::i32::MAX {
            result
        } else {
            -1
        }
    }
}
