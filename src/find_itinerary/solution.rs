use crate::*;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let n = tickets.len();
        let mut edges = HashMap::new();
        for (i, mut edge) in tickets.into_iter().enumerate() {
            let source = edges.entry(edge.remove(0)).or_insert(Vec::new());
            source.push((edge.pop().unwrap(), i));
        }
        for (_, destinations) in edges.iter_mut() {
            destinations.sort();
        }
        let mut current = vec![String::from("JFK")];
        let mut result = None;
        fn helper(
            left_tickets: &mut HashSet<usize>,
            edges: &HashMap<String, Vec<(String, usize)>>,
            current: &mut Vec<String>,
            result: &mut Option<Vec<String>>,
        ) {
            if left_tickets.len() == 0 {
                result.replace(current.clone());
                return;
            }
            if let Some(destinations) = edges.get(current.last().unwrap()) {
                for (destination, i) in destinations {
                    if left_tickets.remove(i) {
                        current.push(destination.clone());
                        helper(left_tickets, edges, current, result);
                        if result.is_some() {
                            return;
                        }
                        left_tickets.insert(*i);
                        current.pop();
                    }
                }
            }
        }
        let mut left_tickets = (0..n).collect();
        helper(&mut left_tickets, &edges, &mut current, &mut result);
        result.unwrap_or(vec![])
    }
}
