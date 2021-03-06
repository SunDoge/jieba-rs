// extern crate regex;

use regex::{CaptureMatches, Captures, Regex};
use std::env;

pub struct SplitCaptures<'r, 't> {
    finder: CaptureMatches<'r, 't>,
    text: &'t str,
    last: usize,
    caps: Option<Captures<'t>>,
}

impl<'r, 't> SplitCaptures<'r, 't> {
    pub fn new(re: &'r Regex, text: &'t str) -> SplitCaptures<'r, 't> {
        SplitCaptures {
            finder: re.captures_iter(text),
            text: text,
            last: 0,
            caps: None,
        }
    }
}

#[derive(Debug)]
pub enum SplitState<'t> {
    Unmatched(&'t str),
    Captured(Captures<'t>),
}

impl<'r, 't> Iterator for SplitCaptures<'r, 't> {
    type Item = SplitState<'t>;

    fn next(&mut self) -> Option<SplitState<'t>> {
        if let Some(caps) = self.caps.take() {
            return Some(SplitState::Captured(caps));
        }
        match self.finder.next() {
            None => if self.last >= self.text.len() {
                None
            } else {
                let s = &self.text[self.last..];
                self.last = self.text.len();
                Some(SplitState::Unmatched(s))
            },
            Some(caps) => {
                let m = caps.get(0).unwrap();
                let unmatched = &self.text[self.last..m.start()];
                self.last = m.end();
                self.caps = Some(caps);
                Some(SplitState::Unmatched(unmatched))
            }
        }
    }
}

// pub struct Seg {
//     seg: char,
// }

// pub fn strdecode(sentence: &str) {}

// pub fn resolve_filename() {}

pub fn char_slice(sentence: &str, start: usize, end: usize) -> &str {
    &sentence[sentence.char_indices().nth(start).unwrap().0..
                  sentence.char_indices().nth(end - 1).unwrap().0 +
                      sentence.char_indices().nth(end - 1).unwrap().1.len_utf8()]
}

pub fn get_module_res(res: &Vec<&str>) -> String {
    let mut cwd = env::current_dir().unwrap();
    cwd.push("src");
    for path in res {
        cwd.push(path);
    }
    cwd.to_str().unwrap().to_string()
}