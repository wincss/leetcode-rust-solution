use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Trie {
    child: HashMap<char, Rc<RefCell<Trie>>>,
    end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
impl Trie {
    /** Initialize your data structure here. */
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            child: HashMap::new(),
            end: false,
        }
    }

    /** Inserts a word into the trie. */
    #[allow(dead_code)]
    pub fn insert(&mut self, word: String) {
        let mut current = None;
        for c in word.chars() {
            match current.take() {
                None => {
                    self.child
                        .entry(c)
                        .or_insert(Rc::new(RefCell::new(Trie::new())));
                    current.replace(Rc::clone(&self.child[&c]));
                }
                Some(v) => {
                    let mut curr = v.borrow_mut();
                    let child = curr
                        .child
                        .entry(c)
                        .or_insert(Rc::new(RefCell::new(Trie::new())));
                    current.replace(Rc::clone(child));
                }
            }
        }
        match current.take() {
            None => {
                self.end = true;
            }
            Some(v) => {
                v.borrow_mut().end = true;
            }
        }
    }

    /** Returns if the word is in the trie. */
    #[allow(dead_code)]
    pub fn search(&self, word: String) -> bool {
        let mut current = None;
        for c in word.chars() {
            match current.take() {
                None => {
                    if self.child.contains_key(&c) {
                        current.replace(Rc::clone(&self.child[&c]));
                    } else {
                        return false;
                    }
                }
                Some(v) => {
                    let curr = v.borrow();
                    if curr.child.contains_key(&c) {
                        current.replace(Rc::clone(&curr.child[&c]));
                    } else {
                        return false;
                    }
                }
            }
        }
        match current.take() {
            None => self.end,
            Some(v) => v.borrow().end,
        }
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    #[allow(dead_code)]
    pub fn starts_with(&self, prefix: String) -> bool {
        let mut current = None;
        for c in prefix.chars() {
            match current.take() {
                None => {
                    if self.child.contains_key(&c) {
                        current.replace(Rc::clone(&self.child[&c]));
                    } else {
                        return false;
                    }
                }
                Some(v) => {
                    let curr = v.borrow();
                    if curr.child.contains_key(&c) {
                        current.replace(Rc::clone(&curr.child[&c]));
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}
