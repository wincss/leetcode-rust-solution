use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Trie {
    child: HashMap<char, Rc<RefCell<Trie>>>,
    end: bool,
}

impl Trie {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            child: HashMap::new(),
            end: false,
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, word: &str) {
        if word.len() == 0 {
            self.end = true;
            return;
        }
        let c = word.chars().next().unwrap();
        self.child
            .entry(c)
            .or_insert(Rc::new(RefCell::new(Trie::new())))
            .borrow_mut()
            .insert(&word[1..])
    }

    #[allow(dead_code)]
    pub fn search(&self, word: &str) -> bool {
        if word.len() == 0 {
            self.end
        } else if word.starts_with('.') {
            self.child.values().any(|v| v.borrow().search(&word[1..]))
        } else {
            let c = word.chars().next().unwrap();
            self.child.contains_key(&c) && self.child[&c].borrow().search(&word[1..])
        }
    }
}

pub struct WordDictionary {
    trie: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { trie: Trie::new() }
    }

    #[allow(dead_code)]
    pub fn add_word(&mut self, word: String) {
        self.trie.insert(&word);
    }

    #[allow(dead_code)]
    pub fn search(&self, word: String) -> bool {
        self.trie.search(&word)
    }
}
