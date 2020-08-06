use crate::*;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Trie {
    pub child: HashMap<u8, Rc<RefCell<Trie>>>,
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
        for &i in word.as_bytes() {
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
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        fn is_palindrome(s: &str) -> bool {
            let s = s.as_bytes();
            let n = s.len();
            for i in 0..n >> 1 {
                if s[i] != s[n - i - 1] {
                    return false;
                }
            }
            true
        }
        let mut ans = vec![];
        let mut t = Trie::new();
        for (i, s) in words.iter().enumerate() {
            t.insert(&s.chars().rev().collect::<String>(), i);
        }
        let t = Rc::new(RefCell::new(t));
        for (i, s) in words.iter().enumerate() {
            let ending = &t.borrow().ending;
            if !ending.is_empty() && (!ending.contains(&i) || ending.len() > 1) && is_palindrome(s)
            {
                for &j in ending.iter() {
                    if i != j {
                        ans.push(vec![i as i32, j as i32]);
                    }
                }
            }

            let mut current = Some(Rc::clone(&t));
            for (j, c) in s.as_bytes().iter().enumerate() {
                let now = current.take().unwrap();
                let now = now.borrow();
                match now.child.get(c) {
                    None => {
                        break;
                    }
                    Some(v) => {
                        current = Some(Rc::clone(v));
                    }
                }
                let ending = &current.as_ref().unwrap().borrow().ending;
                if !ending.is_empty()
                    && (!ending.contains(&i) || ending.len() > 1)
                    && is_palindrome(&s[j + 1..])
                {
                    for &j in ending.iter() {
                        if i != j {
                            ans.push(vec![i as i32, j as i32]);
                        }
                    }
                }
            }
        }
        let mut t = Trie::new();
        for (i, s) in words.iter().enumerate() {
            t.insert(s, i);
        }
        let t = Rc::new(RefCell::new(t));
        for (i, s) in words.iter().enumerate() {
            let s = &s.chars().rev().collect::<String>();
            let ending = &t.borrow().ending;
            if !ending.is_empty() && (!ending.contains(&i) || ending.len() > 1) && is_palindrome(s)
            {
                for &j in ending.iter() {
                    if i != j {
                        ans.push(vec![j as i32, i as i32]);
                    }
                }
            }
            let n = s.len();
            let mut current = Some(Rc::clone(&t));
            for (j, c) in s.as_bytes().iter().enumerate() {
                if j == n - 1 {
                    break;
                }
                let now = current.take().unwrap();
                let now = now.borrow();
                match now.child.get(c) {
                    None => {
                        break;
                    }
                    Some(v) => {
                        current = Some(Rc::clone(v));
                    }
                }
                let ending = &current.as_ref().unwrap().borrow().ending;
                if !ending.is_empty()
                    && (!ending.contains(&i) || ending.len() > 1)
                    && is_palindrome(&s[j + 1..])
                {
                    for &j in ending.iter() {
                        if i != j {
                            ans.push(vec![j as i32, i as i32]);
                        }
                    }
                }
            }
        }
        ans
    }
}
