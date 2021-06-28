use crate::*;

use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let mut station_bus = HashMap::new();
        let mut bus_link = HashMap::new();
        for (bus, stations) in routes.iter().enumerate() {
            for &station in stations.iter() {
                let buses = station_bus.entry(station).or_insert(HashSet::new());
                for &i in buses.iter() {
                    bus_link.entry(i).or_insert(HashSet::new()).insert(bus);
                    bus_link.entry(bus).or_insert(HashSet::new()).insert(i);
                }
                buses.insert(bus);
            }
        }
        if !station_bus.contains_key(&source) || !station_bus.contains_key(&target) {
            return -1;
        }
        let mut q = VecDeque::new();
        let mut v = HashSet::new();
        for &i in station_bus[&source].iter() {
            q.push_back((1, i));
            v.insert(i);
        }
        while let Some((step, current)) = q.pop_front() {
            if station_bus[&target].contains(&current) {
                return step;
            }
            if bus_link.contains_key(&current) {
                for &next in bus_link[&current].iter() {
                    if v.contains(&next) {
                        continue;
                    }
                    q.push_back((step + 1, next));
                    v.insert(next);
                }
            }
        }
        -1
    }
}
