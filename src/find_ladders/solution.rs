use crate::*;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
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
            return vec![];
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
        let mut result = vec![];
        let mut current_path = vec![];
        fn recover_path(
            destination: usize,
            show_in_path: bool,
            path_node: &Vec<HashSet<usize>>,
            words: &Vec<Vec<char>>,
            current_path: &mut Vec<String>,
            result: &mut Vec<Vec<String>>,
        ) {
            if show_in_path {
                current_path.insert(0, words[destination].iter().collect());
            }
            if destination == 0 {
                result.push(current_path.clone());
            } else {
                for &node in path_node[destination].iter() {
                    recover_path(node, !show_in_path, path_node, words, current_path, result);
                }
            }
            if show_in_path {
                current_path.remove(0);
            }
        }
        recover_path(
            *destination.unwrap(),
            true,
            &path_node,
            &words,
            &mut current_path,
            &mut result,
        );
        result
    }
}
