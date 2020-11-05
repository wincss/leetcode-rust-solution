use crate::*;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        fn get_node(
            word: &Vec<char>,
            words: &mut Vec<Vec<char>>,
            words_rev: &mut HashMap<Vec<char>, usize>,
        ) -> usize {
            let idx = words_rev.entry(word.clone()).or_insert(words.len());
            if *idx == words.len() {
                words.push(word.clone());
            }
            *idx
        }
        let mut word_list = word_list;
        word_list.insert(0, begin_word);

        let mut words = vec![];
        let mut words_rev = HashMap::new();
        let mut edges = HashMap::new();

        for word in word_list {
            let mut word: Vec<char> = word.chars().collect();
            let word_id = get_node(&word, &mut words, &mut words_rev);
            for i in 0..word.len() {
                let c = word[i];
                word[i] = '.';
                let pattern_id = get_node(&word, &mut words, &mut words_rev);
                edges
                    .entry(word_id)
                    .or_insert(HashSet::new())
                    .insert(pattern_id);
                edges
                    .entry(pattern_id)
                    .or_insert(HashSet::new())
                    .insert(word_id);
                word[i] = c;
            }
        }

        let end_word: Vec<char> = end_word.chars().collect();
        let destination = words_rev.get(&end_word);
        if destination.is_none() {
            return 0;
        }
        let mut short_path = vec![std::i32::MAX; words.len()];
        let mut path_node = vec![HashSet::new(); words.len()];
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, 0_usize)));
        while let Some(Reverse((dist, node))) = queue.pop() {
            if short_path[node] < dist {
                continue;
            }
            for &next_node in edges[&node].iter() {
                if short_path[next_node] > dist + 1 {
                    short_path[next_node] = dist + 1;
                    queue.push(Reverse((dist + 1, next_node)));
                }
                if short_path[next_node] == dist + 1 {
                    path_node[next_node].insert(node);
                }
            }
        }
        let destination = *destination.unwrap();
        if short_path[destination] < std::i32::MAX {
            short_path[destination] / 2 + 1
        } else {
            0
        }
    }
}
