use crate::*;

use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut edges = HashMap::new();
        for edge in tickets.into_iter() {
            let source = edges.entry(edge[0].clone()).or_insert(Vec::new());
            source.push(edge[1].clone());
        }
        for (_, destinations) in edges.iter_mut() {
            destinations.sort();
            destinations.reverse();
        }
        // println!("{:?}", edges);
        fn helper(now: String, edges: &mut HashMap<String, Vec<String>>, result: &mut Vec<String>) {
            while let Some(destination) = edges.get_mut(&now).and_then(|v| v.pop()) {
                helper(destination, edges, result);
            }
            result.push(now);
        }
        let mut result = vec![];
        helper(String::from("JFK"), &mut edges, &mut result);
        result.into_iter().rev().collect()
    }
}
