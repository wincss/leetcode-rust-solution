use crate::*;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mw = max_width as usize;
        let mut result = vec![];
        let mut line_width = 0;
        let mut line_words: Vec<String> = vec![];
        for w in words.into_iter() {
            if line_width + line_words.len() + w.len() > mw {
                if line_words.len() == 1 {
                    let mut line = String::new();
                    line.push_str(&line_words[0]);
                    line.push_str(&" ".repeat(mw - line_width));
                    result.push(line);
                } else {
                    let mut line = String::new();
                    let space = (mw - line_width) / (line_words.len() - 1);
                    let mut rest = (mw - line_width) % (line_words.len() - 1);
                    line.push_str(&line_words[0]);
                    for w in line_words[1..].iter() {
                        line.push_str(&" ".repeat(space));
                        if rest > 0 {
                            line.push(' ');
                            rest -= 1;
                        }
                        line.push_str(w);
                    }
                    result.push(line);
                }
                line_width = 0;
                line_words.clear();
            }
            line_width += w.len();
            line_words.push(w);
        }
        assert!(!line_words.is_empty());
        let mut line = String::new();
        line.push_str(&line_words[0]);
        for w in line_words[1..].iter() {
            line.push(' ');
            line.push_str(w);
        }
        line.push_str(&" ".repeat(mw - line.len()));
        result.push(line);
        result
    }
}
