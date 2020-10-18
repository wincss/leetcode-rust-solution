use crate::*;

struct SkipIter<I: Iterator> {
    skip_char: I::Item,
    skipped: usize,
    s: I,
}

impl<I> SkipIter<I>
where
    I: Iterator,
{
    fn new(s: I, skip_char: I::Item) -> Self {
        SkipIter {
            skip_char,
            skipped: 0,
            s,
        }
    }
}

impl<I> Iterator for SkipIter<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.s.next() {
                Some(v) => {
                    if v == self.skip_char {
                        self.skipped += 1;
                    } else if self.skipped > 0 {
                        self.skipped -= 1;
                    } else {
                        return Some(v);
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }
}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s = SkipIter::new(s.chars().rev(), '#');
        let mut t = SkipIter::new(t.chars().rev(), '#');
        loop {
            let sv = s.next();
            let tv = t.next();
            if sv != tv {
                return false;
            } else if sv.is_none() && tv.is_none() {
                return true;
            }
        }
    }
}
