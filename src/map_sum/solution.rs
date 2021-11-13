use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Trie {
    child: HashMap<char, Rc<RefCell<Trie>>>,
    sum: i32,
}

impl Trie {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            child: HashMap::new(),
            sum: 0,
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, word: &str, val_diff: i32) {
        self.sum += val_diff;
        if word.len() == 0 {
            return;
        }
        let c = word.chars().next().unwrap();
        self.child
            .entry(c)
            .or_insert(Rc::new(RefCell::new(Trie::new())))
            .borrow_mut()
            .insert(&word[1..], val_diff)
    }

    #[allow(dead_code)]
    pub fn sum(&self, word: &str) -> i32 {
        if word.len() == 0 {
            self.sum
        } else {
            let c = word.chars().next().unwrap();
            if self.child.contains_key(&c) {
                self.child[&c].borrow().sum(&word[1..])
            } else {
                0
            }
        }
    }
}

pub struct MapSum {
    trie: Trie,
    map: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            trie: Trie::new(),
            map: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, val: i32) {
        let val_diff = val - *self.map.get(&key).unwrap_or(&0);
        self.trie.insert(&key, val_diff);
        self.map.insert(key, val);
    }

    #[allow(dead_code)]
    pub fn sum(&self, prefix: String) -> i32 {
        self.trie.sum(&prefix)
    }
}
