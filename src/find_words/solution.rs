use crate::*;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Trie {
    pub child: HashMap<char, Rc<RefCell<Trie>>>,
    pub ending: HashSet<usize>,
}

impl Trie {
    #[inline]
    pub fn new() -> Self {
        Trie {
            child: HashMap::new(),
            ending: HashSet::new(),
        }
    }

    pub fn insert(&mut self, word: &str, pos: usize) {
        let mut current = None;
        for i in word.chars() {
            match current.take() {
                None => {
                    self.child
                        .entry(i)
                        .or_insert(Rc::new(RefCell::new(Trie::new())));
                    current.replace(Rc::clone(&self.child[&i]));
                }
                Some(v) => {
                    let mut curr = v.borrow_mut();
                    let child = curr
                        .child
                        .entry(i)
                        .or_insert(Rc::new(RefCell::new(Trie::new())));
                    current.replace(Rc::clone(child));
                }
            }
        }
        match current.take() {
            None => {
                self.ending.insert(pos);
            }
            Some(v) => {
                v.borrow_mut().ending.insert(pos);
            }
        }
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        fn dfs(
            x: usize,
            y: usize,
            tree: Rc<RefCell<Trie>>,
            n: usize,
            m: usize,
            board: &mut Vec<Vec<char>>,
            result: &mut HashSet<usize>,
        ) {
            if x > n - 1 || y > m - 1 {
                return;
            }
            let c = board[x][y];
            if !tree.borrow().child.contains_key(&c) {
                return;
            }
            let next = Rc::clone(&tree.borrow().child[&c]);
            for &i in next.borrow().ending.iter() {
                result.insert(i);
            }
            board[x][y] = '#';
            dfs(x + 1, y, Rc::clone(&next), n, m, board, result);
            dfs(x.wrapping_sub(1), y, Rc::clone(&next), n, m, board, result);
            dfs(x, y + 1, Rc::clone(&next), n, m, board, result);
            dfs(x, y.wrapping_sub(1), Rc::clone(&next), n, m, board, result);
            board[x][y] = c;
            if next.borrow().child.is_empty() {
                tree.borrow_mut().child.remove(&c);
            }
        }
        let n = board.len();
        let m = if n > 0 { board[0].len() } else { 0 };
        let mut board = board;
        let mut t = Trie::new();
        for (i, s) in words.iter().enumerate() {
            t.insert(s, i);
        }
        let t = Rc::new(RefCell::new(t));
        let mut result = HashSet::new();
        for i in 0..n {
            for j in 0..m {
                dfs(i, j, Rc::clone(&t), n, m, &mut board, &mut result);
            }
        }
        words
            .into_iter()
            .enumerate()
            .filter_map(|(idx, s)| if result.contains(&idx) { Some(s) } else { None })
            .collect()
    }
}
